use std::sync::atomic::{AtomicU32, Ordering};

struct FixedArray<T> {
    datas: Vec<T>,
    index: AtomicU32,
    size: u32,
    last: u32
}

impl<T> FixedArray<T> {
    fn push(&mut self, data: T) -> bool {
        let mut v = self.index.compare_and_swap(self.size, self.last, Ordering::Relaxed);
        if v == self.size {
            /*
             * cas is true
             * */
            return false;
        } else {
            /*
             * > size
             * */
            v = self.index.compare_and_swap(self.last, self.last, Ordering::Relaxed);
            if v == self.last {
                return false;
            }
            /*
             * < size
             * */
        }
        self.datas[v] = data;
        return true;
    }

    pub fn get(&mut self, index: u32) -> Option<T> {
        let mut v = self.index.compare_and_swap(self.size, self.last, Ordering::Relaxed);
        if v == self.size {
            /*
             * cas is true
             * */
            return None;
        } else {
            /*
             * > size
             * */
            v = self.index.compare_and_swap(self.last, self.last, Ordering::Relaxed);
            if v == self.last {
                return None;
            }
            /*
             * < size
             * */
        }
        Some(self.datas[v])
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

//////////////////////////////////
pub struct LoopArray {
}

impl LoopArray {
    pub fn new() -> Self {
        Self {
        }
    }
}

