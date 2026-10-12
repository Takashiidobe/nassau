use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex, MutexGuard};

use super::value::{
    KIND_ARRAY, KIND_CLOSURE, KIND_REAL, KIND_RECORD, KIND_REF, KIND_STRING, KIND_VECTOR,
};

const MARK: i64 = 0x80;
const MINIMUM_TRIGGER: usize = 4 << 20;

struct Configuration {
    collecting: bool,
    heap: usize,
    stress: usize,
}

fn parse_size(text: &str) -> Option<usize> {
    let (digits, shift) = match text.chars().last()? {
        'k' | 'K' => (&text[..text.len() - 1], 10),
        'm' | 'M' => (&text[..text.len() - 1], 20),
        'g' | 'G' => (&text[..text.len() - 1], 30),
        _ => (text, 0),
    };
    digits.parse::<usize>().ok()?.checked_mul(1 << shift)
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
    Configuration {
        collecting: plan == "MarkSweep",
        heap: parse_size(&heap).expect("invalid NASSAU_GC_HEAP"),
        stress,
    }
});

#[derive(Default)]
struct AddressHasher(u64);

impl Hasher for AddressHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = (self.0 ^ u64::from(*byte)).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        }
    }

    fn write_usize(&mut self, word: usize) {
        let product = (word as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        self.0 = product ^ (product >> 32);
    }
}

struct Heap {
    objects: HashSet<usize, BuildHasherDefault<AddressHasher>>,
    used: usize,
    since_collection: usize,
    trigger: usize,
    allocations: usize,
}

static HEAP: LazyLock<Mutex<Heap>> = LazyLock::new(|| {
    Mutex::new(Heap {
        objects: HashSet::default(),
        used: 0,
        since_collection: 0,
        trigger: MINIMUM_TRIGGER,
        allocations: 0,
    })
});

fn heap() -> MutexGuard<'static, Heap> {
    HEAP.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
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

pub fn allocate(bytes: usize, header: i64) -> *mut i64 {
    let mut heap = heap();
    let configuration = &*CONFIGURATION;
    heap.allocations += 1;
    if configuration.collecting {
        let stressed =
            configuration.stress != 0 && heap.allocations.is_multiple_of(configuration.stress);
        let full = heap.used.saturating_add(bytes) > configuration.heap;
        if stressed || full || heap.since_collection.saturating_add(bytes) > heap.trigger {
            heap.collect();
        }
    }
    if heap.used.saturating_add(bytes) > configuration.heap {
        drop(heap);
        super::out_of_memory();
    }
    let layout = Layout::from_size_align(bytes, 8).expect("valid Nassau object layout");
    let address = unsafe { alloc_zeroed(layout) }.cast::<i64>();
    if address.is_null() {
        drop(heap);
        super::out_of_memory();
    }
    unsafe { address.write(header) };
    heap.objects.insert(address as usize);
    heap.used += bytes;
    heap.since_collection += bytes;
    address
}

unsafe fn read_word(address: usize) -> usize {
    unsafe { (address as *const u64).read() as usize }
}

fn object_header(object: usize) -> i64 {
    unsafe { (object as *const i64).read() }
}

impl Heap {
    fn collect(&mut self) {
        let mut pending = Vec::new();
        for slot in frame_roots().into_iter().chain(vm_roots()) {
            self.mark(unsafe { read_word(slot) }, &mut pending);
        }
        for &root in HOST_ROOTS.lock().unwrap().iter() {
            self.mark(root as usize, &mut pending);
        }
        while let Some(object) = pending.pop() {
            let header = object_header(object);
            let length = (header >> 8) as usize;
            let first = match header & 0x7f {
                KIND_RECORD | KIND_ARRAY | KIND_REF | KIND_VECTOR => 0,
                KIND_CLOSURE => {
                    let code = unsafe { read_word(object + 8) };
                    for slot in code_roots(code) {
                        self.mark(unsafe { read_word(slot) }, &mut pending);
                    }
                    1
                }
                KIND_STRING | KIND_REAL => continue,
                _ => unreachable!("valid Nassau object kind"),
            };
            for field in first..length {
                let word = unsafe { read_word(object + (field + 1) * 8) };
                self.mark(word, &mut pending);
            }
        }
        let mut freed = 0;
        self.objects.retain(|&object| {
            let header = object_header(object);
            if header & MARK != 0 {
                unsafe { (object as *mut i64).write(header & !MARK) };
                return true;
            }
            let bytes =
                physical_size(header >> 8, header & 0x7f).expect("valid Nassau object header");
            unsafe {
                dealloc(
                    object as *mut u8,
                    Layout::from_size_align_unchecked(bytes, 8),
                )
            };
            freed += bytes;
            false
        });
        self.used -= freed;
        self.since_collection = 0;
        self.trigger = self.used.max(MINIMUM_TRIGGER);
    }

    fn mark(&self, word: usize, pending: &mut Vec<usize>) {
        if word == 0 || word & 7 != 0 || !self.objects.contains(&word) {
            return;
        }
        let header = object_header(word);
        if header & MARK == 0 {
            unsafe { (word as *mut i64).write(header | MARK) };
            pending.push(word);
        }
    }
}

fn vm_roots() -> Vec<usize> {
    let mut roots = Vec::new();
    for index in 0..super::value::BUILTIN_EXCEPTIONS.len() {
        roots.push(unsafe { (&raw mut super::IDENTITIES).cast::<i64>().add(index) } as usize);
    }
    roots.push(&raw mut super::RAISED as usize);
    roots.push(&raw mut super::UNCAUGHT as usize);
    roots.extend(GLOBAL_ROOTS.lock().unwrap().iter().copied());
    roots
}

static ROOT_FRAME: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn push_roots(frame: *mut u64, count: u64, code: u64) {
    unsafe {
        frame.write(ROOT_FRAME.load(Ordering::Relaxed) as u64);
        frame.add(1).write(count);
        frame.add(2).write(code);
    }
    ROOT_FRAME.store(frame as usize, Ordering::Release);
}

pub fn reset_frames() {
    ROOT_FRAME.store(0, Ordering::Release);
}

pub unsafe fn pop_roots(frame: *mut u64) {
    assert_eq!(ROOT_FRAME.load(Ordering::Relaxed), frame as usize);
    ROOT_FRAME.store(unsafe { frame.read() } as usize, Ordering::Release);
}

fn frame_roots() -> Vec<usize> {
    let mut frame = ROOT_FRAME.load(Ordering::Acquire) as *mut u64;
    let mut slots = Vec::new();
    while !frame.is_null() {
        unsafe {
            let count = frame.add(1).read() as usize;
            slots.extend(code_roots(frame.add(2).read() as usize));
            for index in 0..count {
                slots.push(frame.add(index + 3) as usize);
            }
            frame = frame.read() as usize as *mut u64;
        }
    }
    slots
}

#[repr(C)]
struct HostRoots<const N: usize> {
    previous: u64,
    count: u64,
    code: u64,
    values: [u64; N],
}

struct RootScope(*mut u64);

impl Drop for RootScope {
    fn drop(&mut self) {
        unsafe { pop_roots(self.0) }
    }
}

pub fn with_roots<const N: usize, T>(values: [u64; N], f: impl FnOnce() -> T) -> T {
    let mut frame = HostRoots {
        previous: 0,
        count: N as u64,
        code: 0,
        values,
    };
    let pointer = (&raw mut frame).cast::<u64>();
    unsafe { push_roots(pointer, N as u64, 0) };
    let _scope = RootScope(pointer);
    f()
}

static HOST_ROOTS: Mutex<Vec<u64>> = Mutex::new(Vec::new());

pub fn push_host_root(value: u64) {
    HOST_ROOTS.lock().unwrap().push(value);
}

pub fn pop_host_root() {
    HOST_ROOTS.lock().unwrap().pop();
}

pub fn clear_globals() {
    GLOBAL_ROOTS.lock().unwrap().clear();
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

fn code_roots(code: usize) -> Vec<usize> {
    CODE_ROOTS
        .lock()
        .unwrap()
        .get(&code)
        .into_iter()
        .flatten()
        .copied()
        .collect()
}
