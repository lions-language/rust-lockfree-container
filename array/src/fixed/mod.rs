use std::sync::atomic::{AtomicU32, Ordering};

struct FixedArray<T> {
    datas: Vec<T>,
    index: AtomicU32,
    size: u32,
    last: u32
}

impl<T> FixedArray<T> {
    fn push(&mut self, t: T) -> bool {
        let mut v = self.index.compare_and_swap(self.size, self.last, Ordering::Relaxed);
        if v == self.size {
            return false;
        } else {
            /*
             * > size
             * */
            v = self.index.compare_and_swap(self.last, self.last, Ordering::Relaxed);
            if v == self.last {
                return false;
            }
        }
        return true;
    }

    fn pop(&mut self) -> Option<T> {
        unimplemented!();
    }

    pub fn new(size: u32) -> Self {
        Self {
            datas: vec![T; size],
            index: AtomicU32::new(0),
            size: size,
            last: size+1
        }
    }
}

