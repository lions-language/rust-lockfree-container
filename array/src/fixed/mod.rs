struct FixedArray<T> {
    datas: Vec<T>,
    index: 0,
    size: u32
}

impl<T> FixedArray<T> {
    fn push(&mut self, t: T) {
        // if atomic.
    }

    fn pop(&mut self) -> Option<T> {
        unimplemented!();
    }

    pub fn new(size: u32) -> Self {
        Self {
            datas: Vec::new(),
            index: 0,
            size: size
        }
    }
}

