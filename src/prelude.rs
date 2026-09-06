use crate::core::Module;
use crate::infer::{self, TypeTable};
use crate::lower::{self, Part, Source};
use crate::parser::{Parser, Program};

const SOURCE: &str = concat!(include_str!("../basis/list.sml"), "\n");

/// The basis library, written in SML and checked and lowered ahead of the
/// user's program.
pub struct Basis {
    program: Program,
    types: TypeTable,
    source: Source,
}

impl Basis {
    /// Parses the embedded basis and checks it in `types`, which is left
    /// holding its declarations.
    pub fn check(types: &mut infer::Session) -> Self {
        let file = "basis.sml";
        let program = Parser::from_source(SOURCE, file)
            .unwrap_or_else(|error| panic!("the basis does not lex: {error:?}"))
            .parse()
            .unwrap_or_else(|error| panic!("the basis does not parse: {error:?}"));
        let checked = types.check(&program).unwrap_or_else(|(error, span)| {
            panic!("the basis does not check: {error} at {span:?}")
        });
        Self {
            program,
            types: checked.types,
            source: Source {
                file: file.into(),
                text: SOURCE.into(),
                first_line: 1,
            },
        }
    }

    pub fn part(&self) -> Part<'_> {
        Part {
            program: &self.program,
            types: &self.types,
            source: &self.source,
        }
    }

    /// Lowers the basis into `lowering` as a module of its own.
    pub fn lower(
        &self,
        lowering: &mut lower::Session,
        entry: &str,
    ) -> Result<Module, lower::Failure> {
        lowering.lower_parts(&[self.part()], entry)
    }
}
