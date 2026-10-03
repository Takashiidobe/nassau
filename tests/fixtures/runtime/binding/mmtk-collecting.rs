use super::*;
use std::sync::{Condvar, Mutex};

#[derive(Default)]
struct Fixture;

static HEAP: LazyLock<Box<MMTK<Fixture>>> = LazyLock::new(|| {
    let mut builder = MMTKBuilder::new_no_env_vars();
    assert!(builder.set_option("plan", "MarkSweep"));
    assert!(builder.set_option("threads", "1"));
    assert!(builder.set_option("gc_trigger", "FixedHeapSize:32m"));
    memory_manager::mmtk_init(&builder)
});
static MUTATOR_ADDRESS: AtomicUsize = AtomicUsize::new(0);
static PARKED: Mutex<bool> = Mutex::new(false);
static CHANGED: Condvar = Condvar::new();
static ROOTS: Mutex<Vec<TaggedSlot>> = Mutex::new(Vec::new());

impl VMBinding for Fixture {
    type VMObjectModel = Nassau;
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

impl ActivePlan<Fixture> for Fixture {
    fn is_mutator(tls: VMThread) -> bool {
        tls == mutator_thread().0
    }
    fn mutator(_tls: VMMutatorThread) -> &'static mut Mutator<Fixture> {
        unsafe { &mut *(MUTATOR_ADDRESS.load(Ordering::Relaxed) as *mut Mutator<Fixture>) }
    }
    fn mutators<'a>() -> Box<dyn Iterator<Item = &'a mut Mutator<Fixture>> + 'a> {
        Box::new(std::iter::once(Self::mutator(mutator_thread())))
    }
    fn number_of_mutators() -> usize {
        1
    }
}

impl Collection<Fixture> for Fixture {
    fn stop_all_mutators<F>(_tls: VMWorkerThread, mut visitor: F)
    where
        F: FnMut(&'static mut Mutator<Fixture>),
    {
        let mut parked = PARKED.lock().unwrap();
        while !*parked {
            parked = CHANGED.wait(parked).unwrap();
        }
        visitor(Self::mutator(mutator_thread()));
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
    fn spawn_gc_thread(_tls: VMThread, ctx: GCThreadContext<Fixture>) {
        std::thread::spawn(move || {
            let GCThreadContext::Worker(worker) = ctx;
            memory_manager::start_worker(&HEAP, VMWorkerThread(mutator_thread().0), worker);
        });
    }
    fn out_of_memory(_tls: VMThread, _kind: AllocationError) {
        super::super::out_of_memory()
    }
}

impl Scanning<Fixture> for Fixture {
    fn scan_object<SV: SlotVisitor<TaggedSlot>>(
        tls: VMWorkerThread,
        object: ObjectReference,
        visitor: &mut SV,
    ) {
        <Nassau as Scanning<Nassau>>::scan_object(tls, object, visitor);
    }
    fn notify_initial_thread_scan_complete(_partial: bool, _tls: VMWorkerThread) {}
    fn scan_roots_in_mutator_thread(
        _tls: VMWorkerThread,
        _mutator: &'static mut Mutator<Fixture>,
        _factory: impl RootsWorkFactory<TaggedSlot>,
    ) {
    }
    fn scan_vm_specific_roots(
        _tls: VMWorkerThread,
        mut factory: impl RootsWorkFactory<TaggedSlot>,
    ) {
        let mut roots = ROOTS.lock().unwrap().clone();
        roots.extend(frame_roots());
        factory.create_process_roots_work(roots);
    }
    fn supports_return_barrier() -> bool {
        false
    }
    fn prepare_for_roots_re_scanning() {}
}

impl ReferenceGlue<Fixture> for Fixture {
    type FinalizableType = ObjectReference;
    fn clear_referent(_object: ObjectReference) {}
    fn get_referent(_object: ObjectReference) -> Option<ObjectReference> {
        None
    }
    fn set_referent(_object: ObjectReference, _referent: ObjectReference) {}
    fn enqueue_references(_references: &[ObjectReference], _tls: VMWorkerThread) {}
}

fn alloc(length: i64, kind: i64) -> *mut i64 {
    let bytes = physical_size(length, kind).unwrap();
    let mutator = Fixture::mutator(mutator_thread());
    let semantics = allocation_semantics(&HEAP, bytes);
    let address = memory_manager::alloc(mutator, bytes, 8, 0, semantics);
    unsafe {
        std::ptr::write_bytes(address.to_mut_ptr::<u8>(), 0, bytes);
        address
            .to_mut_ptr::<i64>()
            .write(super::super::value::header(length, kind));
    }
    let object = ObjectReference::from_raw_address(address).unwrap();
    memory_manager::post_alloc(mutator, object, bytes, semantics);
    address.to_mut_ptr()
}

fn collect() {
    assert!(memory_manager::handle_user_collection_request(
        &HEAP,
        mutator_thread()
    ));
}

fn managed(pointer: *mut i64) -> bool {
    memory_manager::is_mmtk_object(Address::from_ptr(pointer)).is_some()
}

pub(super) fn verify() {
    let mutator = memory_manager::bind_mutator(&HEAP, mutator_thread());
    MUTATOR_ADDRESS.store(Box::into_raw(mutator) as usize, Ordering::Relaxed);
    memory_manager::initialize_collection(&HEAP, mutator_thread().0);

    let live = alloc(3, KIND_RECORD);
    let empty = alloc(0, KIND_RECORD);
    let no_captures = alloc(1, KIND_CLOSURE);
    let large_record = alloc(20000, KIND_RECORD);
    let large_string = alloc(100000, KIND_STRING);
    let cell = alloc(1, KIND_REF);
    let closure = alloc(2, KIND_CLOSURE);
    let dead = alloc(1, KIND_REF);
    let dead_other = alloc(1, KIND_REF);
    let raw_code = alloc(0, KIND_RECORD);
    let raw_real = alloc(0, KIND_RECORD);
    let raw_string = alloc(0, KIND_RECORD);
    let real = alloc(1, KIND_REAL);
    let string = alloc(8, KIND_STRING);
    unsafe {
        live.add(1).write(cell as i64);
        live.add(2).write(cell as i64);
        live.add(3).write(closure as i64);
        cell.add(1).write(live as i64);
        closure.add(1).write(raw_code as i64);
        closure.add(2).write(cell as i64);
        dead.add(1).write(dead_other as i64);
        dead_other.add(1).write(dead as i64);
        real.add(1).write(raw_real as i64);
        string.add(1).write(raw_string as i64);
        large_record.add(20000).write(cell as i64);
        large_string.add(1).write(raw_string as i64);
        no_captures.add(1).write(raw_code as i64);
    }
    let static_literal = [super::super::value::header(1, KIND_REAL), 0];
    let mut roots = [
        live as usize,
        real as usize,
        string as usize,
        large_record as usize,
        large_string as usize,
        empty as usize,
        no_captures as usize,
        static_literal.as_ptr() as usize,
        0,
        1,
        3,
    ];
    *ROOTS.lock().unwrap() = roots
        .iter_mut()
        .map(|root| TaggedSlot(Address::from_mut_ptr(root)))
        .collect();
    let stack_only = alloc(1, KIND_REF);
    let host_only = alloc(1, KIND_REF);
    unsafe {
        stack_only.add(1).write(stack_only as i64);
        host_only.add(1).write(host_only as i64);
    }
    let mut frame = [0usize, 0, 0, stack_only as usize];
    unsafe { push_roots(frame.as_mut_ptr(), 1, 0) };
    with_roots([host_only as usize], collect);
    assert!(managed(stack_only));
    assert!(managed(host_only));
    unsafe { pop_roots(frame.as_mut_ptr()) };
    assert_eq!(ROOT_FRAME.load(Ordering::Acquire), 0);
    for pointer in [
        live,
        cell,
        closure,
        real,
        string,
        large_record,
        large_string,
        empty,
        no_captures,
    ] {
        assert!(managed(pointer));
    }
    for pointer in [dead, dead_other, raw_code, raw_real, raw_string] {
        assert!(!managed(pointer));
    }
    unsafe {
        assert_eq!(live.add(1).read(), live.add(2).read());
        assert_eq!(cell.add(1).read(), live as i64);
    }
    roots.fill(0);
    collect();
    assert!(!managed(stack_only));
    assert!(!managed(host_only));
    for pointer in [
        live,
        cell,
        closure,
        real,
        string,
        large_record,
        large_string,
        empty,
        no_captures,
    ] {
        assert!(!managed(pointer));
    }
    memory_manager::destroy_mutator(Fixture::mutator(mutator_thread()));
    unsafe {
        drop(Box::from_raw(
            MUTATOR_ADDRESS.swap(0, Ordering::Relaxed) as *mut Mutator<Fixture>
        ));
    }
    println!("MMTk MarkSweep retained shared cycles and reclaimed dead cycles");
}
