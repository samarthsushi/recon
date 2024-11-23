pub struct Arena {
    data: Vec<String>,
}

impl Arena {
    pub fn new() -> Self {
        Arena { data: Vec::new() }
    }

    pub fn alloc(&mut self, s: String) -> &str {
        self.data.push(s);
        // reason for unsafe: we just pushed a value before this so last() will return something
        unsafe { self.data.last().unwrap_unchecked().as_str() }
    }
}