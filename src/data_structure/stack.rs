//! # Stack Data Structure
//!
//! A stack is a linear data structure that follows the Last-In-First-Out (LIFO) principle. 
//! Elements are added and removed from the top, or "topmost" end of the stack. It operates like a collection of items stacked on top of each other, where the last item added is the first to be removed. 
//! Stacks are used in various applications, such as managing function calls in programming, tracking navigation history, and implementing undo operations. 
//! Common operations on a stack include push (adding an element), pop (removing the top element), and peek (viewing the top element without removal).

#[derive(Debug)]
pub struct Stack<T>
{
    data: Vec<T>,
}

impl<T> Stack<T>
{
    /// Creates a new empty stack.
    /// 
    /// # Examples
    ///
    /// ```
    /// let stack: Stack<i32> = Stack::new();
    /// ```
    pub fn new() -> Self
    {
        Stack {
            data: Vec::new(),
        }
    }

    /// Adds elements to the stack.
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new();
    /// stack.push(1);
    /// assert!(!is_null());
    /// ```
    pub fn push(&mut self, element: T)
    {
        self.data.push(element)
    }

    /// Removes and returns an element from the stack.
    /// Returns `None` if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new();
    /// stack.push(1);
    /// let element = stack.pop();
    /// assert_eq!(element, Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T>
    {
        self.data.pop()
    }

    /// Checks if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new();
    /// stack.push(1);
    /// assert!(!stack.is_null());
    /// ```
    pub fn is_null(&mut self) -> bool
    {
        self.data.is_empty()
    }

    /// Peeks at the top element of the stack without removing it.
    /// Returns `None` if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut stack: Stack<i32> = Stack::new();
    /// stack.push(1);
    /// assert_eq!(stack.peek(), Some(&1));
    /// ```
    pub fn peek(&mut self) -> Option<&T>
    {
        self.data.last()
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn test_stack()
    {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let element_pop = stack.pop();
        assert_eq!(element_pop.unwrap(), 3);

        assert!(!stack.is_null());

        let element_pop = stack.pop();
        let element_pop = stack.pop();

        assert_eq!(stack.peek(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
    }
}