//! # Linked List Data Structure
//!
//! A linked list is a linear data structure consisting of nodes, where each node contains an element and a reference (or link) to the next node in the sequence. 
//! Unlike arrays, linked lists do not require contiguous memory allocation. This allows for efficient insertions and deletions at any point in the list.
//! Linked lists can be singly linked (each node points to the next) or doubly linked (each node points to both the next and previous nodes), enabling traversal in both directions.
//! They are commonly used in scenarios where dynamic size changes and efficient insertions/deletions are important, like implementing stacks, queues, and hash tables.
#[derive(Debug)]
pub struct Node<T>
{
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct List<T>
{
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> List<T>
{
    /// Creates a new empty Linked List.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::linked_list::List;
    /// let mut list: List<i32> = List::new();
    /// ```
    pub fn new() -> Self
    {
        List{ 
            head: None,
            len: 0,
        }
    }
    
    /// Adds elements to the Linked List.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::linked_list::List;
    /// let mut list: List<i32> = List::new();
    /// list.push(1);
    /// assert_eq!(list.size(), 1);
    /// ```
    pub fn push(&mut self, data: T)
    {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        
        self.head = Some(new_node);
        self.len += 1;
    }

    /// Removes and returns an element from the Linked List.
    /// Returns `None` if the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::linked_list::List;
    /// let mut list: List<i32> = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// assert_eq!(list.pop(), Some(3));
    /// ```
    pub fn pop(&mut self) -> Option<T>
    {
        self.head
            .take()
            .map(|boxed_node| {
                let node = *boxed_node;
                self.head = node.next;
                self.len -= 1;
                node.data
            })
    }

    /// Checks if the list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use RustyDSA::data_structure::linked_list::List;
    /// let mut list: List<i32> = List::new();
    /// list.push(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool 
    {
        self.head.is_none()
    }

    pub fn size(&self) -> usize
    {
        self.len
    }
}

#[cfg(test)]
mod test
{
    use super::List as LinkedList;
    #[test]
    fn test_linked_list()
    {
        let mut list = LinkedList::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.size(), 3);
    }
}
