#[derive(Debug)]
pub struct Stack<T>
{
    data: Vec<T>,
}

impl<T> Stack<T>
{
    pub fn new() -> Self
    {
        Stack {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, element: T)
    {
        self.data.push(element)
    }

    pub fn pop(&mut self) -> Option<T>
    {
        self.data.pop()
    }

    pub fn is_null(&mut self) -> bool
    {
        self.data.is_empty()
    }

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