# multi_stack_queue

A crate for stack-allocated fixed-length multiqueues. A multiqueue is an array of a given number of queues, each able to be accessed independently.



# Usage

The generic definition is the following :

```rust
MultiStackQueue<T, const N: usize, const M: usize>
```

With :

* `T` - type contained in the queues
* `N` - length of each queue
* `M` - number of queues

# Example usecases

* When writing a simple micro-kernel, the scheduler may need some sort of multiple Round-Robins.
Having it allocated on the stack removes the need for a heap allocator, which can be useful
when working on this kind of ressource-limited target.

# Examples

```rust
use multi_stack_queue::MultiStackQueue;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TestStruct {
    a: usize,
    b: bool,   
}

let mut msq: MultiStackQueue<TestStruct, 16, 8> = MultiStackQueue::new();
let value = TestStruct { a: 42, b: false };

msq.push(7, value).unwrap();

assert_eq!(msq.pop(7).unwrap(), value);
```

# Roadmap

* Using arrays of `Option<T>` requires that `T` implements the `Copy` trait, which may not be the case. A different approach is to use default values instead of `Option::None` to initialize the arrays. This way, `T` must need not implement `Copy` but `Default`, which may be beneficial in some usecases. Another idea would be to make use of the `MaybeUnInit` type.
* Add options in the generic definition of `MultiStackQueue` to enable the user to specify the procedure in case of a `push` on a full queue or a `pop` on an empty queue. For instance, one could wish trying to push an element to a full queue would simply push it to the following queue (and same thing when trying to `pop` an element). This would add a sort of "spill mechanism"
