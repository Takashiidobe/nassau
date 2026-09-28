//! Which names are constructors where, for the checks that run before type
//! inference (scope and match analysis).
//!
//! A capitalised name in a pattern is only a constructor if a `datatype` (or
//! the initial basis) put it in scope; otherwise it is an ordinary variable.
//! Each constructor remembers the other constructors of its datatype, so match
//! analysis knows when a set of rules covers every one of them.

use std::rc::Rc;

use crate::parser::{DataBinding, DeclKind, ExceptionKind};

/// The constructors of one datatype with the number of arguments each takes
/// in a pattern (`::` takes two: its head and tail).
pub type Family = Rc<Vec<(String, usize)>>;

#[derive(Clone)]
struct Entry {
    name: String,
    /// `None` for constructors of open types, such as exceptions.
    family: Option<Family>,
}

#[derive(Clone)]
pub struct Constructors {
    entries: Vec<Entry>,
    datatypes: Vec<(String, Family)>,
}

/// What a scope declared, set aside to be restored after an enclosing scope ends.
pub struct Declared {
    entries: Vec<(String, Option<Family>)>,
    datatypes: Vec<(String, Family)>,
}

/// A point to return to when a scope ends.
#[derive(Clone, Copy)]
pub struct Mark(usize, usize);

fn family(constructors: &[(&str, usize)]) -> Family {
    Rc::new(
        constructors
            .iter()
            .map(|(name, arity)| ((*name).to_owned(), *arity))
            .collect(),
    )
}

impl Constructors {
    /// The constructors of the initial basis.
    pub fn new() -> Self {
        let mut constructors = Self {
            entries: Vec::new(),
            datatypes: Vec::new(),
        };
        let builtin = [
            ("bool", family(&[("true", 0), ("false", 0)])),
            ("list", family(&[("nil", 0), ("::", 2)])),
            ("option", family(&[("NONE", 0), ("SOME", 1)])),
            (
                "order",
                family(&[("LESS", 0), ("EQUAL", 0), ("GREATER", 0)]),
            ),
        ];
        for (name, family) in builtin {
            constructors.add_family(name, &family);
        }
        for name in [
            "Div",
            "Overflow",
            "Match",
            "Bind",
            "Empty",
            "Subscript",
            "Fail",
        ] {
            constructors.entries.push(Entry {
                name: name.into(),
                family: None,
            });
        }
        constructors
    }

    fn add_family(&mut self, name: &str, family: &Family) {
        for (constructor, _) in family.iter() {
            self.entries.push(Entry {
                name: constructor.clone(),
                family: Some(family.clone()),
            });
        }
        self.datatypes.push((name.to_owned(), family.clone()));
    }

    fn lookup(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().rev().find(|entry| entry.name == name)
    }

    /// Qualified names (`Foo.Bar`) are assumed to be constructors, as modules
    /// are not supported yet.
    pub fn is_constructor(&self, name: &str) -> bool {
        name.contains('.') || self.lookup(name).is_some()
    }

    /// The constructors of the same datatype as `name`, if it is closed.
    pub fn family_of(&self, name: &str) -> Option<Family> {
        self.lookup(name)?.family.clone()
    }

    pub fn mark(&self) -> Mark {
        Mark(self.entries.len(), self.datatypes.len())
    }

    pub fn release(&mut self, mark: Mark) {
        self.entries.truncate(mark.0);
        self.datatypes.truncate(mark.1);
    }

    /// Removes what was declared since `mark` and returns it, so a caller can
    /// restore some of it after releasing an enclosing scope.
    pub fn take_since(&mut self, mark: Mark) -> Declared {
        Declared {
            entries: self
                .entries
                .drain(mark.0..)
                .map(|entry| (entry.name, entry.family))
                .collect(),
            datatypes: self.datatypes.drain(mark.1..).collect(),
        }
    }

    pub fn restore(&mut self, kept: Declared) {
        for (name, family) in kept.entries {
            self.entries.push(Entry { name, family });
        }
        self.datatypes.extend(kept.datatypes);
    }

    /// Forgets the constructors (but not the type names) added since `mark`,
    /// as at the end of an `abstype`.
    pub fn hide_constructors(&mut self, mark: Mark, until: Mark) {
        self.entries.drain(mark.0..until.0);
    }

    pub fn declare_datatypes(&mut self, bindings: &[DataBinding]) {
        // Every name of the group is in scope in the others, so families are
        // built before any is added.
        let families: Vec<(&str, Family)> = bindings
            .iter()
            .map(|binding| {
                let constructors = binding
                    .constructors
                    .iter()
                    .map(|constructor| {
                        (
                            constructor.value.name.clone(),
                            usize::from(constructor.value.argument.is_some()),
                        )
                    })
                    .collect();
                (binding.name.as_str(), Rc::new(constructors))
            })
            .collect();
        for (name, family) in families {
            self.add_family(name, &family);
        }
    }

    /// Adds what a datatype replication makes visible.
    pub fn declare_copy(&mut self, name: &str, original: &str) {
        let family = self
            .datatypes
            .iter()
            .rev()
            .find(|(known, _)| known == original)
            .map(|(_, family)| family.clone());
        if let Some(family) = family {
            self.add_family(name, &family);
        }
    }

    /// Records the constructors a declaration brings into scope.
    pub fn declare(&mut self, declaration: &DeclKind) {
        match declaration {
            DeclKind::Datatype { bindings, .. } => self.declare_datatypes(bindings),
            DeclKind::DatatypeCopy { name, original } => self.declare_copy(name, original),
            DeclKind::Exception(bindings) => {
                // Exceptions are constructors of one open type: any number
                // more can be declared, so no set of them is ever exhaustive.
                for binding in bindings {
                    debug_assert!(matches!(
                        binding.kind,
                        ExceptionKind::Fresh(_) | ExceptionKind::Copy(_)
                    ));
                    self.entries.push(Entry {
                        name: binding.name.clone(),
                        family: None,
                    });
                }
            }
            _ => {}
        }
    }
}
