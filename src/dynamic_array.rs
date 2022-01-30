pub struct DynamicArray<T> {
    vals: *mut T,
    pub index: usize,
    pub capacity: usize,
}

impl<T> DynamicArray<T> {
    pub fn new(capacity: usize) -> Self{
        Self {
            vals: Vec::with_capacity(capacity).as_mut_ptr(),
            index: 0,
            capacity,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.index >= self.capacity {
            self.capacity *= 2;
            let ptr: *mut T = Vec::with_capacity(self.capacity).as_mut_ptr();
            for i in 0..(self.capacity - 1) {
                unsafe {
                    ptr.add(i).write(self.vals.add(i).read());
                }
            }
            self.vals = ptr;
        }
        unsafe {
            self.vals.add(self.index).write(val);
        }
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.index <= 0 {
            None
        } else {
            self.index -= 1;
            unsafe {
                Some(self.vals.add(self.index).read())
            }
        }
    }

    pub fn len(&self) -> usize {
        self.index
    }

    pub fn is_empty(&self) -> bool {
        self.index == 0
    }
}
