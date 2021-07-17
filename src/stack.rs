pub struct Stack<T> {
    vals: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { vals: vec![] }
    }
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { vals: vec }
    }

    pub fn push(&mut self, val: T) {
        self.vals.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vals.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.vals.is_empty()
    }
}
