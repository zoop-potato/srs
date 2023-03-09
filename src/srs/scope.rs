/// Fast and simple way to have both start and end indexs into a string AND all the functinality of &str
pub struct Scope<'a> {
    string: &'a str,
    start_index: usize,
}

impl Scope<'a> {
    
}