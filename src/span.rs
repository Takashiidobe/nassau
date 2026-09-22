use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loc {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span<T> {
    pub start: Loc,
    pub end: Loc,
    pub value: T,
}

impl<T> Span<T> {
    pub fn new(start: Loc, end: Loc, value: T) -> Self {
        Self { start, end, value }
    }
    pub fn source_span(&self) -> miette::SourceSpan {
        (
            self.start.offset,
            self.end.offset.saturating_sub(self.start.offset),
        )
            .into()
    }
}
