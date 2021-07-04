//! A crate for stack-allocated fixed-length multiqueues. A multiqueue is an array of a given number of queues,
//! each able to be accessed independently.
//!
//! In term, this crate should include a feature that enables the user to specify what the multiqueue must do
//! in the case the `pop` or `push` method cannot operate (e.g. empty or full individual queue.).
//! For instance, one could wish the operation is, in such a case, applied to the following queue.
//!
//! This crate was motivated by the creation of a multiple-round-robin-based scheduler in a toy micro-kernel.
//! Each queue holds all the threads within the same priority level.
//! Attempting to create a new thread in an already full priority level would simply decrease its priority
//! until a suitable non-full queue is found.



/// Errors that may be encountered during use of the [`MultiStackQueue`]
///
/// * `QueueFull` - Returned by the `push` method when trying to append a value to a queue that is already full
/// * `QueueEmpty` - Returned by the `pop` method when trying to pop a value from an empty queue
/// * `QueueIndexOutOfBounds` - When trying to access a queue beyond the multiqueue
/// * `UnknownError` - This should never happen. Used for development
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MSQError {
    QueueFull,
    QueueEmpty,
    QueueIndexOutOfBounds,
    UnknowmError,
}

/// An abstract structure containin multiple stack-allocated bounded queues.
///
/// Each queue is stored as an `[Option<T>; N]` and the multiqueue stores
//// the complete data in an `[[Option<T>; N]; M].

///
/// # Usage
///
/// The generic definition is the following :
///
/// ```ignore
/// MultiStackQueue<T, const N: usize, const M: usize>
/// ```
///
/// With :
///
/// * `T` - type contained in the queues
/// * `N` - length of each queue
/// * `M` - number of queues
///
/// # Example usecases
///
/// * When writing a simple micro-kernel, the scheduler may need some sort of multiple Round-Robins.
/// Having it allocated on the stack removes the need for a heap allocator, which can be useful
/// when working on this kind of ressource-limited target.
///
/// # Examples
///
/// ```
/// use multi_stack_queue::MultiStackQueue;
///
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// struct TestStruct {
///     a: usize,
///     b: bool,   
/// }
///
/// let mut msq: MultiStackQueue<TestStruct, 16, 8> = MultiStackQueue::new();
/// let value = TestStruct { a: 42, b: false };
///
/// msq.push(7, value).unwrap();
///
/// assert_eq!(msq.pop(7).unwrap(), value);
/// ```
///
/// # Roadmap
///
/// Using arrays of `Option<T>` requires that `T` implements the `Copy` trait, which may not be the case.
/// A different approach is to use default values instead of `Option::None` to initialize the arrays.
/// This way, `T` must need not implement `Copy` but `Default`, which may be beneficial in some usecases.
///
/// Another idea would be to make use of the `MaybeUnInit` type.
///
pub struct MultiStackQueue<T, const N: usize, const M: usize> {
    data: [[Option<T>; N]; M],
    ins: [usize; M],
    outs: [usize; M],
    empty: [bool; M],
}

impl<T: Copy, const N: usize, const M: usize> MultiStackQueue<T, N, M> {
    /// Returns a new empty multiqueue.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_stack_queue::MultiStackQueue;
    /// // Returns a fresh empty multiqueue containing 8 queues of `usize` with size 16
    /// let a: MultiStackQueue<usize, 16, 8> = MultiStackQueue::new();
    ///
    /// #[derive(Clone, Copy)]
    /// struct TestStruct {
    ///     a: usize,
    ///     b: bool    
    /// }
    ///
    /// let random_data = TestStruct { a: 42, b: false };
    ///
    /// let msq: MultiStackQueue<TestStruct, 4, 2> = MultiStackQueue::new();
    /// ```
    ///
    pub fn new() -> Self {
        MultiStackQueue {
            data: [[None; N]; M],
            ins: [0usize; M],
            outs: [0usize; M],
            empty: [true; M],
        }
    }
    /// Appends a value to the multiqueue.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_stack_queue::MultiStackQueue;
    ///
    /// #[derive(Clone, Copy)]
    /// struct TestStruct {
    ///     a: usize,
    ///     b: bool    
    /// }
    ///
    /// let random_data = TestStruct { a: 42, b: false };
    ///
    /// let mut msq: MultiStackQueue<TestStruct, 4, 2> = MultiStackQueue::new();
    ///
    /// msq.push(0, random_data).unwrap();
    /// ```
    ///
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
    /// Pops a value from the multiqueue.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_stack_queue::MultiStackQueue;
    ///
    /// #[derive(Clone, Copy)]
    /// struct TestStruct {
    ///     a: usize,
    ///     b: bool    
    /// }
    ///
    /// let random_data = TestStruct { a: 42, b: false };
    ///
    /// let mut msq: MultiStackQueue<TestStruct, 4, 2> = MultiStackQueue::new();
    ///
    /// msq.push(0, random_data).unwrap();
    /// msq.pop(0).unwrap();
    /// ```
    ///
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
    /// Returns whether a particular queue is empty
    /// # Examples
    ///
    /// ```
    /// use multi_stack_queue::MultiStackQueue;
    ///
    /// let mut msq: MultiStackQueue<usize, 4, 2> = MultiStackQueue::new();
    ///
    /// assert!(!msq.is_full(0));
    /// for _ in 0..4 {
    ///     msq.push(0, 0);
    /// }
    /// assert!(msq.is_full(0));
    /// ```
    ///
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

    #[test]
    fn fifo() {
        let mut a: MultiStackQueue<usize, 16, 32> = MultiStackQueue::new();
        a.push(0, 1).unwrap();
        a.push(0, 2).unwrap();
        assert_eq!(a.pop(0).unwrap(), 1);
        assert_eq!(a.pop(0).unwrap(), 2);
    }
}
