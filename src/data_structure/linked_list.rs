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

impl<T> LinkedList<T>
{
    fn new() -> Self
    {
        List{ 
            head: None,
            len: 0,
        }
    }

    pub fn push(&mut self, data: T)
    {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        
        self.head = Some(new_node);
        self.len += 1;
    }

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
    mod queue;
    mod stack;
    mod heap;
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
