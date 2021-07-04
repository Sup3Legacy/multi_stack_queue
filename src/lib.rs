#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MSQError {
    QueueFull,
    QueueEmpty,
    QueueIndexOutOfBounds,
    UnknowmError,
}

pub struct MultiStackQueue<T, const N: usize, const M: usize> {
    data: [[Option<T>; N]; M],
    ins: [usize; M],
    outs: [usize; M],
    empty: [bool; M],
}

impl<T: Copy, const N: usize, const M: usize> MultiStackQueue<T, N, M> {
    pub fn new() -> Self {
        MultiStackQueue {
            data: [[None; N]; M],
            ins: [0usize; M],
            outs: [0usize; M],
            empty: [true; M],
        }
    }
}

impl<T, const N: usize, const M: usize> MultiStackQueue<T, N, M> {
    pub fn push(&mut self, id: usize, value: T) -> Result<(), MSQError> {
        if id >= M {
            return Err(MSQError::QueueIndexOutOfBounds);
        }
        self.try_and_push(id, value)
    }
    fn try_and_push(&mut self, id: usize, value: T) -> Result<(), MSQError> {
        if self.ins[id] == self.outs[id] && !self.empty[id] {
            // Queue is full
            Err(MSQError::QueueFull)
        } else {
            self.data[id][self.ins[id]] = Some(value);
            self.ins[id] = (self.ins[id] + 1) % N;
            self.empty[id] = false;
            Ok(())
        }
    }
    pub fn pop(&mut self, id: usize) -> Result<T, MSQError> {
        if id >= M {
            return Err(MSQError::QueueIndexOutOfBounds);
        }
        self.try_and_pop(id)
    }
    fn try_and_pop(&mut self, id: usize) -> Result<T, MSQError> {
        if self.empty[id] {
            Err(MSQError::QueueEmpty)
        } else {
            // TODO The unwrap is not ideal
            let res = self.data[id][self.outs[id]].take().unwrap();
            self.outs[id] = (self.outs[id] + 1) % N;
            if self.outs[id] == self.ins[id] {
                self.empty[id] = true;
            }
            Ok(res)
        }
    }
    pub fn is_full(&self, id: usize) -> bool {
        !self.empty[id] && self.ins[id] == self.outs[id]
    }
    pub fn is_empty(&self, id: usize) -> bool {
        self.empty[id]
    }
}

#[cfg(test)]
mod tests {
    use crate::MultiStackQueue;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TestStruct {
        a: usize,
        b: bool,
    }

    #[test]
    fn creation() {
        let a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
    }

    #[test]
    fn push_once() {
        let mut a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
        let val = TestStruct { a: 42, b: true };
        a.push(12, val).unwrap();
    }

    #[test]
    fn push_and_pop_once() {
        let mut a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
        let val = TestStruct { a: 42, b: true };
        a.push(12, val).unwrap();
        assert_eq!(a.pop(12).unwrap(), val);
        assert!(a.is_empty(12));
    }

    #[test]
    #[should_panic]
    fn push_and_pop_twice() {
        let mut a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
        let val = TestStruct { a: 42, b: true };
        a.push(12, val).unwrap();
        a.pop(12).unwrap();
        a.pop(12).unwrap();
    }

    #[test]
    #[should_panic]
    fn pop_empty() {
        let mut a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
        a.pop(12).unwrap();
    }

    #[test]
    fn fill() {
        let mut a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
        let val = TestStruct { a: 42, b: true };
        for _ in 0..16 {
            a.push(12, val).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn fill_overflow() {
        let mut a: MultiStackQueue<TestStruct, 16, 32> = MultiStackQueue::new();
        let val = TestStruct { a: 42, b: true };
        for _ in 0..=16 {
            a.push(12, val).unwrap();
        }
    }
}
