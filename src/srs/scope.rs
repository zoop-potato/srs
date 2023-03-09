/// Fast and simple way to have both start and end indexs into a string AND all the functinality of &str
#[derive(Debug, Clone, Copy)]
pub struct Scope<'a> {
    string: &'a str,
    start_index: usize,
}

impl<'a> Scope<'a> {
    pub fn new(start_index: usize, string: &'a str) -> Self {
        Scope { string: string, start_index: start_index }
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn end_index(&self) -> usize {
        self.start_index + self.string.len()
    }

    pub fn range(&self) -> std::ops::RangeInclusive<usize> {
        self.start_index..=self.end_index()
    }
}

impl <'a> From<Scope<'a>> for &'a str {
    fn from(val: Scope<'a>) -> Self {
        val.string
    }
}