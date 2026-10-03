#[expect(dead_code, reason = "the fixture uses the shared runtime layout")]
#[path = "../../../src/value.rs"]
mod value;

static mut IDENTITIES: [i64; value::BUILTIN_EXCEPTIONS.len()] =
    [0; value::BUILTIN_EXCEPTIONS.len()];
static mut RAISED: i64 = 0;
static mut UNCAUGHT: i64 = 0;

fn out_of_memory() -> ! {
    std::process::abort()
}

mod binding {
    include!("../../../runtime/gc.rs");

    pub fn verify() {
        for (length, kind, bytes) in [
            (0, KIND_RECORD, 8),
            (1, KIND_CLOSURE, 16),
            (0, KIND_STRING, 16),
            (7, KIND_STRING, 16),
            (8, KIND_STRING, 24),
            (15, KIND_STRING, 24),
            (16, KIND_STRING, 32),
            (1, KIND_REAL, 16),
            (1, KIND_REF, 16),
        ] {
            assert_eq!(physical_size(length, kind), Some(bytes));
            let pointer = allocate(bytes, super::value::header(length, kind));
            assert_eq!(pointer as usize & 7, 0);
            let object = memory_manager::is_mmtk_object(Address::from_ptr(pointer)).unwrap();
            assert_eq!(
                <Nassau as ObjectModel<Nassau>>::get_current_size(object),
                bytes
            );
            assert_eq!(
                <Nassau as ObjectModel<Nassau>>::get_size_when_copied(object),
                bytes
            );
            assert_eq!(
                <Nassau as ObjectModel<Nassau>>::ref_to_object_start(object),
                Address::from_ptr(pointer)
            );
            for field in 1..bytes / 8 {
                assert_eq!(unsafe { pointer.add(field).read() }, 0);
            }
        }
        for (length, kind) in [
            (-1, KIND_RECORD),
            (i64::MAX, KIND_RECORD),
            (i64::MAX, KIND_STRING),
            (0, KIND_CLOSURE),
            (0, KIND_REAL),
            (2, KIND_REF),
            (0, 255),
        ] {
            assert_eq!(physical_size(length, kind), None);
        }

        let mut global = 0usize;
        register_global((&raw mut global) as usize);
        assert!(
            GLOBAL_ROOTS
                .lock()
                .unwrap()
                .contains(&((&raw mut global) as usize))
        );
        replace_globals(&[]);
        assert!(GLOBAL_ROOTS.lock().unwrap().is_empty());
        register_code_global(0x123400, (&raw mut global) as usize);
        assert_eq!(
            code_roots(0x123400),
            vec![TaggedSlot(Address::from_mut_ptr(&mut global))]
        );
        CODE_ROOTS.lock().unwrap().clear();

        let child = allocate(16, super::value::header(1, KIND_REF));
        let object = memory_manager::is_mmtk_object(Address::from_ptr(child)).unwrap();
        let static_real = [super::value::header(1, KIND_REAL), 0];
        let mut word = 0usize;
        let slot = TaggedSlot(Address::from_mut_ptr(&mut word));
        for value in [
            0,
            1,
            3,
            usize::MAX,
            usize::MAX & !7,
            8,
            child as usize + 8,
            static_real.as_ptr() as usize,
        ] {
            word = value;
            assert_eq!(slot.load(), None, "{word:#x}");
        }
        slot.store(object);
        assert_eq!(slot.load(), Some(object));
        assert_eq!(word, child as usize);

        struct Visitor(Vec<TaggedSlot>);
        impl SlotVisitor<TaggedSlot> for Visitor {
            fn visit_slot(&mut self, slot: TaggedSlot) {
                self.0.push(slot);
            }
        }
        let tls = VMWorkerThread(VMThread(OpaquePointer::UNINITIALIZED));
        for (kind, length, fields) in [
            (KIND_RECORD, 0, vec![]),
            (KIND_RECORD, 3, vec![1, 2, 3]),
            (KIND_CLOSURE, 1, vec![]),
            (KIND_CLOSURE, 3, vec![2, 3]),
            (KIND_REF, 1, vec![1]),
            (KIND_STRING, 0, vec![]),
            (KIND_STRING, 16, vec![]),
            (KIND_REAL, 1, vec![]),
        ] {
            let block = allocate(
                physical_size(length, kind).unwrap(),
                super::value::header(length, kind),
            );
            let object = memory_manager::is_mmtk_object(Address::from_ptr(block)).unwrap();
            for field in 1..<Nassau as ObjectModel<Nassau>>::get_current_size(object) / 8 {
                unsafe { block.add(field).write(child as i64) };
            }
            let mut visitor = Visitor(Vec::new());
            Nassau::scan_object(tls, object, &mut visitor);
            let expected: Vec<_> = fields
                .into_iter()
                .map(|field| TaggedSlot(Address::from_ptr(block) + field as usize * 8))
                .collect();
            assert_eq!(visitor.0, expected);
            assert!(visitor.0.iter().all(|slot| slot.load()
                == Some(memory_manager::is_mmtk_object(Address::from_ptr(child)).unwrap())));
        }
    }
    #[path = "mmtk-collecting.rs"]
    mod collecting;

    pub fn verify_collection() {
        collecting::verify();
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--collect") {
        binding::verify_collection();
    } else if std::env::args().any(|arg| arg == "--allocation") {
        binding::verify();
    } else {
        for mode in ["--allocation", "--collect"] {
            assert!(
                std::process::Command::new(std::env::current_exe().unwrap())
                    .arg(mode)
                    .env("NASSAU_GC_PLAN", "NoGC")
                    .env_remove("NASSAU_GC_STRESS")
                    .status()
                    .unwrap()
                    .success()
            );
        }
        println!("MMTk object model fixture passed");
    }
}
