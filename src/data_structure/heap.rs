#[derive(Debug)]
pub struct Heap<T>
{
    data: Vec<T>,
}

impl<T: PartialOrd> Heap<T>
{
    pub fn new() -> Self
    {
        Heap {
            data: Vec::new()
        }
    }

    pub fn push(&mut self, element: T)
    {
        self.data.push(element);
    }

    pub fn heapify(&mut self, n: usize, i: usize)
    {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && self.data[left] > self.data[largest]
        {
            largest = left;
        }

        if right < n && self.data[right] > self.data[largest]
        {
            largest = right;
        }

        if largest != i 
        {
            self.data.swap(i, largest);

            self.heapify(n, largest);
        }
    }

    pub fn heap_sort(&mut self)
    {
        let n = self.data.len();

        for i in(0..n / 2).rev()
        {
            self.heapify(n, i);
        }

        for i in (0..n).rev()
        {
            self.data.swap(0, i);
            self.heapify(i, 0);
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    fn test_heap()
    {
        let mut heap: Heap<i32> = Heap::new();
        heap.push(4);
        heap.push(5);
        heap.push(2);
        heap.push(10);
        heap.push(12);
        heap.push(1);
        heap.push(9);

        heap.heap_sort();

        assert!(heap.data.windows(2).all(|w| w[0] <= w[1]));
    }
}