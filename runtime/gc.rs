use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, LazyLock, Mutex, Once};

use super::value::{
    KIND_ARRAY, KIND_CLOSURE, KIND_REAL, KIND_RECORD, KIND_REF, KIND_STRING, KIND_VECTOR,
};

use mmtk::memory_manager;
use mmtk::util::alloc::AllocationError;
use mmtk::util::copy::{CopySemantics, GCWorkerCopyContext};
use mmtk::util::{
    Address, ObjectReference, OpaquePointer, VMMutatorThread, VMThread, VMWorkerThread,
};
use mmtk::vm::slot::{Slot, UnimplementedMemorySlice};
use mmtk::vm::{
    ActivePlan, Collection, GCThreadContext, ObjectModel, ReferenceGlue, RootsWorkFactory,
    Scanning, SlotVisitor, VMBinding, VMGlobalLogBitSpec, VMLocalForwardingBitsSpec,
    VMLocalForwardingPointerSpec, VMLocalLOSMarkNurserySpec, VMLocalMarkBitSpec,
};
use mmtk::{AllocationSemantics, MMTK, MMTKBuilder, Mutator};

#[derive(Default)]
struct Nassau;

struct Configuration {
    plan: String,
    heap: String,
    stress: usize,
}

static CONFIGURATION: LazyLock<Configuration> = LazyLock::new(|| {
    let plan = std::env::var("NASSAU_GC_PLAN").unwrap_or_else(|_| "MarkSweep".into());
    assert!(
        matches!(plan.as_str(), "NoGC" | "MarkSweep"),
        "NASSAU_GC_PLAN must be NoGC or MarkSweep"
    );
    let heap = std::env::var("NASSAU_GC_HEAP").unwrap_or_else(|_| "32m".into());
    let stress = std::env::var("NASSAU_GC_STRESS")
        .map(|value| {
            value
                .parse()
                .expect("NASSAU_GC_STRESS must be an allocation interval")
        })
        .unwrap_or(0);
    Configuration { plan, heap, stress }
});
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

static MMTK: LazyLock<Box<MMTK<Nassau>>> = LazyLock::new(|| {
    let mut builder = MMTKBuilder::new_no_env_vars();
    assert!(builder.set_option("plan", &CONFIGURATION.plan));
    assert!(builder.set_option("threads", "1"));
    assert!(
        builder.set_option(
            "gc_trigger",
            &format!("FixedHeapSize:{}", CONFIGURATION.heap)
        ),
        "invalid NASSAU_GC_HEAP"
    );
    memory_manager::mmtk_init(&builder)
});

thread_local! {
    static THREAD: Box<u8> = Box::new(0);
    static MUTATOR: RefCell<BoundMutator> =
        RefCell::new(BoundMutator::new());
}

struct BoundMutator(Box<Mutator<Nassau>>);

static MUTATOR_ADDRESS: AtomicUsize = AtomicUsize::new(0);
static MUTATOR_THREAD: AtomicUsize = AtomicUsize::new(0);
static COLLECTION_STARTED: Once = Once::new();
static PARKED: Mutex<bool> = Mutex::new(false);
static CHANGED: Condvar = Condvar::new();

impl BoundMutator {
    fn new() -> Self {
        let mut mutator = memory_manager::bind_mutator(&MMTK, mutator_thread());
        let pointer = (&raw mut *mutator) as usize;
        assert!(
            MUTATOR_ADDRESS
                .compare_exchange(0, pointer, Ordering::Release, Ordering::Relaxed)
                .is_ok(),
            "only one SML mutator may execute at a time"
        );
        MUTATOR_THREAD.store(
            mutator_thread().0.0.to_address().as_usize(),
            Ordering::Release,
        );
        COLLECTION_STARTED
            .call_once(|| memory_manager::initialize_collection(&MMTK, mutator_thread().0));
        Self(mutator)
    }
}

impl Drop for BoundMutator {
    fn drop(&mut self) {
        memory_manager::destroy_mutator(&mut self.0);
        MUTATOR_THREAD.store(0, Ordering::Release);
        MUTATOR_ADDRESS.store(0, Ordering::Release);
    }
}

fn mutator_thread() -> VMMutatorThread {
    THREAD.with(|token| {
        VMMutatorThread(VMThread(OpaquePointer::from_address(Address::from_ptr(
            &**token,
        ))))
    })
}

pub fn physical_size(length: i64, kind: i64) -> Option<usize> {
    let length = usize::try_from(length).ok()?;
    if length > (i64::MAX as usize >> 8) {
        return None;
    }
    let words = match kind {
        KIND_STRING => (length / 8).checked_add(1)?,
        KIND_RECORD | KIND_ARRAY | KIND_VECTOR => length,
        KIND_CLOSURE if length >= 1 => length,
        KIND_REAL | KIND_REF if length == 1 => 1,
        _ => return None,
    };
    words
        .checked_add(1)?
        .checked_mul(8)
        .filter(|bytes| *bytes <= isize::MAX as usize)
}

fn allocation_semantics<VM: VMBinding>(heap: &MMTK<VM>, bytes: usize) -> AllocationSemantics {
    if bytes
        > heap
            .get_plan()
            .constraints()
            .max_non_los_default_alloc_bytes
    {
        AllocationSemantics::Los
    } else {
        AllocationSemantics::Default
    }
}

pub fn allocate(bytes: usize, header: i64) -> *mut i64 {
    MUTATOR.with(|mutator| {
        let mut mutator = mutator.borrow_mut();
        let allocations = ALLOCATIONS.fetch_add(1, Ordering::Relaxed) + 1;
        if CONFIGURATION.plan == "MarkSweep"
            && CONFIGURATION.stress != 0
            && allocations.is_multiple_of(CONFIGURATION.stress)
        {
            memory_manager::handle_user_collection_request(&MMTK, mutator_thread());
        }
        let semantics = allocation_semantics(&MMTK, bytes);
        let address = memory_manager::alloc(&mut mutator.0, bytes, 8, 0, semantics);
        let object =
            ObjectReference::from_raw_address(address).unwrap_or_else(|| super::out_of_memory());
        unsafe {
            std::ptr::write_bytes(address.to_mut_ptr::<u8>(), 0, bytes);
            address.to_mut_ptr::<i64>().write(header);
        }
        memory_manager::post_alloc(&mut mutator.0, object, bytes, semantics);
        address.to_mut_ptr()
    })
}

impl VMBinding for Nassau {
    type VMObjectModel = Self;
    type VMScanning = Self;
    type VMCollection = Self;
    type VMActivePlan = Self;
    type VMReferenceGlue = Self;
    type VMSlot = TaggedSlot;
    type VMMemorySlice = UnimplementedMemorySlice<TaggedSlot>;

    const MIN_ALIGNMENT: usize = 8;
    const MAX_ALIGNMENT: usize = 8;
    const USE_ALLOCATION_OFFSET: bool = false;
    const ALLOC_END_ALIGNMENT: usize = 8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TaggedSlot(Address);

impl Slot for TaggedSlot {
    fn load(&self) -> Option<ObjectReference> {
        let word = unsafe { &*self.0.to_ptr::<AtomicUsize>() }.load(Ordering::Relaxed);
        if word == 0 || word & 7 != 0 {
            return None;
        }
        memory_manager::is_mmtk_object(unsafe { Address::from_usize(word) })
    }

    fn store(&self, object: ObjectReference) {
        unsafe { &*self.0.to_ptr::<AtomicUsize>() }
            .store(object.to_raw_address().as_usize(), Ordering::Relaxed);
    }
}

fn object_header(object: ObjectReference) -> i64 {
    unsafe { object.to_raw_address().to_ptr::<i64>().read() }
}

impl<VM: VMBinding> ObjectModel<VM> for Nassau {
    const GLOBAL_LOG_BIT_SPEC: VMGlobalLogBitSpec = VMGlobalLogBitSpec::side_first();
    const LOCAL_FORWARDING_POINTER_SPEC: VMLocalForwardingPointerSpec =
        VMLocalForwardingPointerSpec::in_header(0);
    const LOCAL_FORWARDING_BITS_SPEC: VMLocalForwardingBitsSpec =
        VMLocalForwardingBitsSpec::side_first();
    const LOCAL_MARK_BIT_SPEC: VMLocalMarkBitSpec = VMLocalMarkBitSpec::side_after(
        <Self as ObjectModel<VM>>::LOCAL_FORWARDING_BITS_SPEC.as_spec(),
    );
    const LOCAL_LOS_MARK_NURSERY_SPEC: VMLocalLOSMarkNurserySpec =
        VMLocalLOSMarkNurserySpec::side_after(
            <Self as ObjectModel<VM>>::LOCAL_MARK_BIT_SPEC.as_spec(),
        );
    const NEED_VO_BITS_DURING_TRACING: bool = true;
    const UNIFIED_OBJECT_REFERENCE_ADDRESS: bool = true;
    const OBJECT_REF_OFFSET_LOWER_BOUND: isize = 0;

    fn copy(
        _from: ObjectReference,
        _semantics: CopySemantics,
        _copy_context: &mut GCWorkerCopyContext<VM>,
    ) -> ObjectReference {
        panic!("moving collectors are not supported")
    }

    fn copy_to(_from: ObjectReference, _to: ObjectReference, _region: Address) -> Address {
        panic!("moving collectors are not supported")
    }

    fn get_reference_when_copied_to(_from: ObjectReference, _to: Address) -> ObjectReference {
        panic!("moving collectors are not supported")
    }

    fn get_current_size(object: ObjectReference) -> usize {
        let header = object_header(object);
        physical_size(header >> 8, header & 0xff).expect("valid Nassau object header")
    }

    fn get_size_when_copied(object: ObjectReference) -> usize {
        <Self as ObjectModel<VM>>::get_current_size(object)
    }

    fn get_align_when_copied(_object: ObjectReference) -> usize {
        8
    }

    fn get_align_offset_when_copied(_object: ObjectReference) -> usize {
        0
    }

    fn get_type_descriptor(_reference: ObjectReference) -> &'static [i8] {
        &[]
    }

    fn ref_to_object_start(object: ObjectReference) -> Address {
        object.to_raw_address()
    }

    fn ref_to_header(object: ObjectReference) -> Address {
        object.to_raw_address()
    }

    fn dump_object(object: ObjectReference) {
        eprintln!(
            "Nassau object {object}: header={:#x}, bytes={}",
            object_header(object),
            <Self as ObjectModel<VM>>::get_current_size(object)
        );
    }
}

impl ActivePlan<Nassau> for Nassau {
    fn is_mutator(tls: VMThread) -> bool {
        let thread = MUTATOR_THREAD.load(Ordering::Acquire);
        thread != 0 && tls.0.to_address().as_usize() == thread
    }

    fn mutator(tls: VMMutatorThread) -> &'static mut Mutator<Nassau> {
        assert!(Self::is_mutator(tls.0));
        let pointer = MUTATOR_ADDRESS.load(Ordering::Acquire) as *mut Mutator<Nassau>;
        assert!(!pointer.is_null());
        unsafe { &mut *pointer }
    }

    fn mutators<'a>() -> Box<dyn Iterator<Item = &'a mut Mutator<Nassau>> + 'a> {
        let pointer = MUTATOR_ADDRESS.load(Ordering::Acquire) as *mut Mutator<Nassau>;
        Box::new(unsafe { pointer.as_mut() }.into_iter())
    }

    fn number_of_mutators() -> usize {
        usize::from(MUTATOR_ADDRESS.load(Ordering::Acquire) != 0)
    }
}

impl Collection<Nassau> for Nassau {
    fn stop_all_mutators<F>(_tls: VMWorkerThread, mut visitor: F)
    where
        F: FnMut(&'static mut Mutator<Nassau>),
    {
        let mut parked = PARKED.lock().unwrap();
        while !*parked {
            parked = CHANGED.wait(parked).unwrap();
        }
        for mutator in Self::mutators() {
            visitor(mutator);
        }
    }

    fn resume_mutators(_tls: VMWorkerThread) {
        *PARKED.lock().unwrap() = false;
        CHANGED.notify_all();
    }

    fn block_for_gc(_tls: VMMutatorThread) {
        let mut parked = PARKED.lock().unwrap();
        *parked = true;
        CHANGED.notify_all();
        while *parked {
            parked = CHANGED.wait(parked).unwrap();
        }
    }

    fn spawn_gc_thread(_tls: VMThread, ctx: GCThreadContext<Nassau>) {
        std::thread::spawn(move || {
            let GCThreadContext::Worker(worker) = ctx;
            memory_manager::start_worker(&MMTK, VMWorkerThread(mutator_thread().0), worker);
        });
    }

    fn out_of_memory(_tls: VMThread, _err_kind: AllocationError) {
        super::out_of_memory()
    }
}

impl Scanning<Nassau> for Nassau {
    fn scan_object<SV: SlotVisitor<TaggedSlot>>(
        _tls: VMWorkerThread,
        object: ObjectReference,
        slot_visitor: &mut SV,
    ) {
        let header = object_header(object);
        let length = (header >> 8) as usize;
        let first = match header & 0xff {
            KIND_RECORD | KIND_ARRAY | KIND_REF | KIND_VECTOR => 0,
            KIND_CLOSURE => {
                let code = unsafe { object.to_raw_address().to_ptr::<usize>().add(1).read() };
                for slot in code_roots(code) {
                    slot_visitor.visit_slot(slot);
                }
                1
            }
            KIND_STRING | KIND_REAL => return,
            _ => unreachable!("valid Nassau object kind"),
        };
        for field in first..length {
            slot_visitor.visit_slot(TaggedSlot(object.to_raw_address() + (field + 1) * 8));
        }
    }

    fn notify_initial_thread_scan_complete(_partial_scan: bool, _tls: VMWorkerThread) {}

    fn scan_roots_in_mutator_thread(
        _tls: VMWorkerThread,
        _mutator: &'static mut Mutator<Nassau>,
        mut factory: impl RootsWorkFactory<TaggedSlot>,
    ) {
        factory.create_process_roots_work(frame_roots());
    }

    fn scan_vm_specific_roots(
        _tls: VMWorkerThread,
        mut factory: impl RootsWorkFactory<TaggedSlot>,
    ) {
        let mut roots = Vec::new();
        for index in 0..super::value::BUILTIN_EXCEPTIONS.len() {
            roots.push(TaggedSlot(Address::from_mut_ptr(unsafe {
                (&raw mut super::IDENTITIES).cast::<i64>().add(index)
            })));
        }
        roots.push(TaggedSlot(Address::from_mut_ptr(&raw mut super::RAISED)));
        roots.push(TaggedSlot(Address::from_mut_ptr(&raw mut super::UNCAUGHT)));
        roots.extend(
            GLOBAL_ROOTS
                .lock()
                .unwrap()
                .iter()
                .map(|address| TaggedSlot(unsafe { Address::from_usize(*address) })),
        );
        factory.create_process_roots_work(roots);
    }

    fn supports_return_barrier() -> bool {
        false
    }

    fn prepare_for_roots_re_scanning() {}
}

impl ReferenceGlue<Nassau> for Nassau {
    type FinalizableType = ObjectReference;

    fn clear_referent(_new_reference: ObjectReference) {}

    fn get_referent(_object: ObjectReference) -> Option<ObjectReference> {
        None
    }

    fn set_referent(_reff: ObjectReference, _referent: ObjectReference) {}

    fn enqueue_references(_references: &[ObjectReference], _tls: VMWorkerThread) {}
}

static ROOT_FRAME: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn push_roots(frame: *mut usize, count: usize, code: usize) {
    unsafe {
        frame.write(ROOT_FRAME.load(Ordering::Relaxed));
        frame.add(1).write(count);
        frame.add(2).write(code);
    }
    ROOT_FRAME.store(frame as usize, Ordering::Release);
}

pub unsafe fn pop_roots(frame: *mut usize) {
    assert_eq!(ROOT_FRAME.load(Ordering::Relaxed), frame as usize);
    ROOT_FRAME.store(unsafe { frame.read() }, Ordering::Release);
}

fn frame_roots() -> Vec<TaggedSlot> {
    let mut frame = ROOT_FRAME.load(Ordering::Acquire) as *mut usize;
    let mut slots = Vec::new();
    while !frame.is_null() {
        unsafe {
            let count = frame.add(1).read();
            slots.extend(code_roots(frame.add(2).read()));
            for index in 0..count {
                slots.push(TaggedSlot(Address::from_mut_ptr(frame.add(index + 3))));
            }
            frame = frame.read() as *mut usize;
        }
    }
    slots
}

#[repr(C)]
struct HostRoots<const N: usize> {
    previous: usize,
    count: usize,
    code: usize,
    values: [usize; N],
}

struct RootScope(*mut usize);

impl Drop for RootScope {
    fn drop(&mut self) {
        unsafe { pop_roots(self.0) }
    }
}

pub fn with_roots<const N: usize, T>(values: [usize; N], f: impl FnOnce() -> T) -> T {
    let mut frame = HostRoots {
        previous: 0,
        count: N,
        code: 0,
        values,
    };
    let pointer = (&raw mut frame).cast::<usize>();
    unsafe { push_roots(pointer, N, 0) };
    let _scope = RootScope(pointer);
    f()
}

static GLOBAL_ROOTS: Mutex<BTreeSet<usize>> = Mutex::new(BTreeSet::new());
static CODE_ROOTS: LazyLock<Mutex<HashMap<usize, BTreeSet<usize>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register_global(address: usize) {
    GLOBAL_ROOTS.lock().unwrap().insert(address);
}

pub fn replace_globals(addresses: &[usize]) {
    *GLOBAL_ROOTS.lock().unwrap() = addresses.iter().copied().collect();
}

pub fn reset_repl_roots() {
    GLOBAL_ROOTS.lock().unwrap().clear();
    CODE_ROOTS.lock().unwrap().clear();
}

pub fn register_code_global(code: usize, address: usize) {
    CODE_ROOTS
        .lock()
        .unwrap()
        .entry(code)
        .or_default()
        .insert(address);
}

fn code_roots(code: usize) -> Vec<TaggedSlot> {
    CODE_ROOTS
        .lock()
        .unwrap()
        .get(&code)
        .into_iter()
        .flatten()
        .map(|address| TaggedSlot(unsafe { Address::from_usize(*address) }))
        .collect()
}
