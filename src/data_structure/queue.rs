//! # Queue Data Structure
//!
//! A queue is a linear data structure that follows the First-In-First-Out (FIFO) principle. Elements are added at the rear, or "enqueue" operation, and removed from 
//! the front, or "dequeue" operation, maintaining their original order. This structure simulates a real-life queue, where the first person to join is the first to be 
//! served. Queues are used in various applications such as task scheduling, breadth-first searches, and print spooling. Basic operations on a queue include enqueue 
//! (adding an element), dequeue (removing the front element), and peek (viewing the front element without removal).

use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> 
{
    data: LinkedList<T>,
}

impl<T> Queue<T>
{
    /// Creates a new empty queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::queue::Queue;
    /// let queue: Queue<i32> = Queue::new();
    /// ```
    pub fn new() -> Self
    {
        Queue {
            data: LinkedList::new(),
        }
    }

    /// Adds an element to the end of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::queue::Queue;
    /// let mut queue: Queue<i32> = Queue::new();
    /// queue.enqueue(1);
    /// assert_eq!(queue.peek(), Some(&1));
    /// ```
    pub fn enqueue(&mut self, element: T)
    {
        self.data.push_back(element)
    }

     /// Removes and returns an element from the front of the queue.
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::queue::Queue;
    /// let mut queue: Queue<i32> = Queue::new();
    /// queue.enqueue(1);
    /// let element = queue.dequeue();
    /// assert_eq!(element, Some(1));
    /// ```
    pub fn dequeue(&mut self) -> Option<T>
    {
        self.data.pop_front()
    }

    /// Checks if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::queue::Queue;
    /// let mut queue: Queue<i32> = Queue::new();
    /// queue.enqueue(1);
    /// assert!(!queue.is_null());
    /// ```
    pub fn is_null(&mut self) -> bool
    {
        self.data.is_empty()
    }

    /// Peeks at the front element of the queue without removing it.
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::queue::Queue;
    /// let mut queue: Queue<i32> = Queue::new();
    /// queue.enqueue(1);
    /// assert_eq!(queue.peek(), Some(&1));
    /// ```
    pub fn peek(&mut self) -> Option<&T>
    {
        self.data.front()
    }
}

#[cfg(test)]
mod test
{
    use super::Queue;
    fn test_queue()
    {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert!(!queue.is_null());

        let element_dequeue = queue.dequeue();
        assert_eq!(element_dequeue, Some(2));

        let element_peek = queue.peek();
        assert_eq!(element_peek, Some(&1));
    }
}