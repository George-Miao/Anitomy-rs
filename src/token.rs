use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TokenRange {
    offset: usize,
    size: usize,
}

impl TokenRange {
    pub fn new(offset: usize, size: usize) -> Self {
        TokenRange { offset, size }
    }

    pub fn as_range(&self) -> Range<usize> {
        self.offset..self.offset + self.size
    }
}
