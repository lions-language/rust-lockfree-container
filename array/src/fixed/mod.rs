struct FixedArray<T> {
    datas: Vec<T>
}

impl<T> FixedArray<T> {
    fn push(&mut self) {
    }

    fn pop(&mut self) -> Option<T> {
        unimplemented!();
    }

    pub fn new() -> Self {
        Self {
        }
    }
}

