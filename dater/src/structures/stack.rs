/// A generic implementation of a Stack data structure using a linked list.
///
/// The Stack follows Last-In-First-Out (LIFO) principles where elements
/// are pushed and popped from the top of the stack.
use crate::LinearData;

/// A node in the linked list implementation of the stack
struct Node<T> {
    value: T,
    previous: Option<Box<Node<T>>>,
}

/// A Stack data structure that can store elements of type T.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the stack
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    ///
    /// # Returns
    ///
    /// A new `Stack<T>` instance.
    pub fn new() -> Self {
        Stack { head: None }
    }

    /// Reverses the order of elements in the stack.
    ///
    /// This operation consumes the original stack.
    ///
    /// # Returns
    ///
    /// A new stack with elements in reverse order.
    pub fn reverse(self) -> Self {
        let mut reversed = Stack::new();

        for x in self {
            reversed.push(x);
        }

        reversed
    }
}

impl<T> LinearData<T> for Stack<T> {
    /// Pushes a value onto the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to push onto the stack
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            previous: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Removes and returns the top element from the stack.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The top element if the stack is not empty
    /// * `None` - If the stack is empty
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.previous;
            node.value
        })
    }

    /// Returns a reference to the top element without removing it.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` - Reference to the top element if the stack is not empty
    /// * `None` - If the stack is empty
    fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Checks if the stack is empty.
    ///
    /// # Returns
    ///
    /// `true` if the stack contains no elements, `false` otherwise.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack { head: None }
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();

        // Test pushing elements
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Test popping elements (LIFO order)
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();

        // Peek on empty stack
        assert_eq!(stack.peek(), None);

        // Push and peek
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));

        // Ensure peek doesn't remove the element
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_iterator() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut items: Vec<i32> = stack.into_iter().collect();
        items.reverse(); // Reverse to check original order
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let reversed = stack.reverse();
        let items: Vec<i32> = reversed.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_default() {
        let stack: Stack<i32> = Stack::default();
        assert!(stack.is_empty());
    }
}
