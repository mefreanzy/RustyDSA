# Document

### [Stack](./src/data_structure/stack.rs)
A stack is a linear data structure that follows the Last-In-First-Out (LIFO) principle. 
Elements are added and removed from the top, or "topmost" end of the stack. It operates like a collection of items stacked on top of each other, where the last item added is the first to be removed. 
Stacks are used in various applications, such as managing function calls in programming, tracking navigation history, and implementing undo operations. 
Common operations on a stack include push (adding an element), pop (removing the top element), and peek (viewing the top element without removal).

* [Wikipedia](https://en.wikipedia.org/wiki/Stack_(abstract_data_type))
* [Geeksforgeeks](https://www.geeksforgeeks.org/stack-data-structure/)
* [Tutorialspoint](https://www.tutorialspoint.com/data_structures_algorithms/stack_algorithm.htm)
* [StudyTonight](https://www.studytonight.com/data-structures/stack-data-structure)

### [Heap](./src/data_structure/heap.rs)
A heap is a binary tree-based data structure where each parent node has a value greater (or smaller, depending on the type of heap) than or equal to the values of its children. This forms a partial ordering among the elements, allowing efficient access to the maximum (or minimum) element. Heaps are commonly used for priority queues, where the highest (or lowest) priority element is quickly accessible. Two types of heaps exist: Max Heap (largest element at the root) and Min Heap (smallest element at the root). Insertion and removal operations maintain the heap property by "heapifying" the tree.

* [Wikipedia](https://en.wikipedia.org/wiki/Heap_(data_structure))
* [Alxolr](https://www.alxolr.com/articles/heap-data-structure-implemented-in-rust-language)
* [Geeksforgeeks](https://www.alxolr.com/articles/heap-data-structure-implemented-in-rust-language)
* [Programiz](https://www.programiz.com/dsa/heap-data-structure)

### [Queue](./src/data_structure/queue.rs)
A queue is a linear data structure that follows the First-In-First-Out (FIFO) principle. Elements are added at the rear, or "enqueue" operation, and removed from the front, or "dequeue" operation, maintaining their original order. This structure simulates a real-life queue, where the first person to join is the first to be served. Queues are used in various applications such as task scheduling, breadth-first searches, and print spooling. Basic operations on a queue include enqueue (adding an element), dequeue (removing the front element), and peek (viewing the front element without removal).

* [Wikipedia](https://en.wikipedia.org/wiki/Queue_(abstract_data_type))
* [Geeksforgeeks](https://www.geeksforgeeks.org/queue-data-structure/)
* [Tutorialspoint](https://www.geeksforgeeks.org/queue-data-structure/)

### [Linked List](./src/data_structure/linked_list.rs)
A linked list is a linear data structure consisting of nodes, where each node contains an element and a reference (or link) to the next node in the sequence. Unlike arrays, linked lists do not require contiguous memory allocation. This allows for efficient insertions and deletions at any point in the list. Linked lists can be singly linked (each node points to the next) or doubly linked (each node points to both the next and previous nodes), enabling traversal in both directions. They are commonly used in scenarios where dynamic size changes and efficient insertions/deletions are important, like implementing stacks, queues, and hash tables.

* [Wikipedia](https://en.wikipedia.org/wiki/Linked_list#:~:text=In%20computer%20science%2C%20a%20linked,which%20together%20represent%20a%20sequence.)
* [Geeksforgeek](https://www.geeksforgeeks.org/data-structures/linked-list/)
* [Tutorialspoint](https://www.tutorialspoint.com/data_structures_algorithms/linked_list_algorithms.htm)
