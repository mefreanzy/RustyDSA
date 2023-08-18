//! # Heap Data Structure
//!
//! A heap is a binary tree-based data structure where each parent node has a value greater (or smaller, depending on the type of heap) than or equal to the values of 
//! its children. This forms a partial ordering among the elements, allowing efficient access to the maximum (or minimum) element. Heaps are commonly used for priority
//! queues, where the highest (or lowest) priority element is quickly accessible. Two types of heaps exist: Max Heap (largest element at the root) and Min Heap 
//! (smallest element at the root). Insertion and removal operations maintain the heap property by "heapifying" the tree.

#[derive(Debug)]
pub struct Heap<T>
{
    data: Vec<T>,
}

impl<T: PartialOrd> Heap<T>
{
    /// Creates a new empty heap.
    ///
    /// # Examples
    ///
    /// ```
    /// let heap: Heap<i32> = Heap::new();
    /// ```
    pub fn new() -> Self
    {
        Heap {
            data: Vec::new()
        }
    }

    /// Pushes an element onto the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut heap: Heap<i32> = Heap::new();
    /// heap.push(4);
    /// heap.push(5);
    /// heap.push(2);
    /// ```
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

    /// Sorts the heap in-place using heap sort algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut heap: Heap<i32> = Heap::new();
    /// heap.push(4);
    /// heap.push(5);
    /// heap.push(2);
    /// heap.heap_sort();
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
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