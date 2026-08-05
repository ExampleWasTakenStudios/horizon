/// A Ring Buffer that lives exclusively on the stack.
#[derive(Debug)]
pub struct Ring<T, const S: usize> {
    buf: [Option<T>; S],
    /// Pointer to the first element in the buffer. This is the one that's popped.
    ///
    /// **This pointer is a monotonic counter and should thus not be used directly to index the buffer. Use [`Self::get_head()`] to get a valid pointer into the buffer.**
    head: usize,
    /// Pointer to the next free slot in the buffer.
    ///
    /// **This pointer is a monotonic counter and should thus not be used directly to index the buffer. Use [`Self::get_tail()`] to get a valid pointer into the buffer.**
    tail: usize,
}

impl<T, const S: usize> Ring<T, S> {
    pub fn new() -> Self {
        let empty_queue: [Option<T>; S] = [const { None }; S];

        Self {
            buf: empty_queue,
            head: 0,
            tail: 0,
        }
    }

    /// Push a struct of type `T` to the end of the buffer.
    ///
    /// Returns [`None`] if the buffer is full and a new element can thus not be pushed.
    pub fn push(&mut self, t: T) -> Option<()> {
        if self.is_full() {
            return None;
        }

        self.buf[self.get_tail()] = Some(t);
        self.tail += 1;

        Some(())
    }

    /// Pop a struct of type `T` from the from the front of the buffer.
    ///
    /// Returns [`Option<T>`].
    pub fn pop(&mut self) -> Option<T> {
        let res= self.buf[self.get_head()].take();
        self.head += 1;

        res
    }

    pub fn get_head(&self) -> usize {
        self.head % S
    }

    pub fn get_tail(&self) -> usize {
        self.tail % S
    }

    pub fn len(&self) -> usize {
        self.tail - self.head
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        self.len() >= S
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_empty() {
        let mut buf = Ring::<u8, 2>::new();

        assert!(buf.is_empty());
        assert!(!buf.is_full());
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.get_head(), 0);
        assert_eq!(buf.get_tail(), 0);

        // Popping a value from the empty buffer should fail (return None)
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn test_push_and_pop_basic() {
        let mut buf = Ring::<u8, 2>::new();
        let initial_head = buf.get_head();
        let initial_tail = buf.get_tail();

        /* PUSHING */
        assert!(buf.push(13).is_some()); // Test pushing works

        assert_eq!(buf.len(), 1); // Test length is updated
        assert_eq!(buf.get_tail(), initial_tail + 1); // Test tail is incremented by one
        assert_eq!(buf.get_head(), initial_head); // Test head is NOT incremented by a push operation

        /* POPPING */
        assert!(buf.pop().is_some()); // Test popping works

        assert_eq!(buf.len(), 0); // Test length is updated back down
        assert_eq!(buf.get_tail(), initial_tail + 1); // Test tail is NOT incremented by a pop operation
        assert_eq!(buf.get_head(), initial_head + 1); // Test head is incremented by one
    }

    #[test]
    fn test_fifo_ordering() {
        let mut buf = Ring::<u8, 3>::new();

        // Calling unwrap() on these methods, ensures the test fails if they fail
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.push(3).unwrap();

        assert_eq!(buf.pop().unwrap(), 1);
        assert_eq!(buf.pop().unwrap(), 2);
        assert_eq!(buf.pop().unwrap(), 3);
    }

    #[test]
    fn test_buffer_full() {
        let mut buf = Ring::<u8, 2>::new();

        buf.push(1).unwrap();
        buf.push(2).unwrap();

        assert!(buf.is_full());
        assert_eq!(buf.len(), 2);

        assert_eq!(buf.push(3), None); // Test that pushing a 3rd element fails
    }

    #[test]
    fn test_wrapping_behavior() {
        let mut buf = Ring::<u8, 3>::new();

        // Push 3 elements - buffer is now full
        buf.push(1).unwrap();
        buf.push(2).unwrap();
        buf.push(3).unwrap();

        // Pop 2 elements -> buffer now has space for two more -> [_, _, 3]
        assert_eq!(buf.pop().unwrap(), 1);
        assert_eq!(buf.pop().unwrap(), 2);

        // Push two more elements. This forces the `tail` to wrap around
        buf.push(4).unwrap(); // Goes into index 0
        buf.push(5).unwrap(); // Goes into index 1

        assert!(buf.is_full());

        // Pop the remaining elements - forces the head to wrap around
        assert_eq!(buf.pop().unwrap(), 3);
        assert_eq!(buf.pop().unwrap(), 4);
        assert_eq!(buf.pop().unwrap(), 5);

        assert!(buf.is_empty());
    }

    #[test]
    fn test_pointer_modulo_logic() {
        let mut buf = Ring::<u8, 2>::new();

        assert_eq!(buf.get_head(), 0);
        assert_eq!(buf.get_tail(), 0);

        buf.push(1).unwrap();

        assert_eq!(buf.get_head(), 0); // Pushing should not modify the head
        assert_eq!(buf.get_tail(), 1); // Pushing should increment the tail by 1

        buf.push(2).unwrap();

        assert_eq!(buf.get_head(), 0); // Pushing should not modify the head
        assert_eq!(buf.get_tail(), 0); // Since the length of `buf` is 2, the tail should wrap around and also be `0`. -> Buffer is full

        assert!(buf.is_full());
    }

    #[test]
    fn test_complete_types() {
        // Ensures that `Option::take()` correctly handles the ownership
        // and drops for types that allocate on the heap, like String.
        let mut buf = Ring::<String, 2>::new();

        buf.push(String::from("hello")).unwrap();
        buf.push(String::from("world")).unwrap();

        assert_eq!(buf.pop().unwrap(), "hello");
        assert_eq!(buf.pop().unwrap(), "world");

        assert!(buf.is_empty());
    }
}
