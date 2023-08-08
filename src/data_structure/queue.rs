use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> 
{
    data: LinkedList<T>,
}

impl<T> Queue<T>
{
    pub fn new() -> Self
    {
        Queue {
            data: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, element: T)
    {
        self.data.push_back(element)
    }

    pub fn dequeue(&mut self) -> Option<T>
    {
        self.data.pop_front()
    }

    pub fn is_null(&mut self) -> bool
    {
        self.data.is_empty()
    }

    pub fn peek(&mut self) -> Option<&T>
    {
        self.data.front()
    }
}

#[cfg(test)]
mod test
{
    use super::*;
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