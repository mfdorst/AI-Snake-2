pub struct PriorityQueue<T: PartialOrd> {
    heap: Vec<T>,
}

impl<T: PartialOrd> PriorityQueue<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut queue = Self { heap: data };
        queue.heapify();
        queue
    }

    pub fn take_min(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let min = self.heap.pop().unwrap();
        self.sift_down(0);
        Some(min)
    }

    pub fn insert(&mut self, element: T) {
        self.heap.push(element);
        let mut i = self.heap.len() - 1;
        // This will overflow if i = 0, but we don't care because we will exit
        let mut parent = (i.wrapping_sub(1)) / 2;
        while i != 0 && self.heap[i] < self.heap[parent] {
            self.heap.swap(i, parent);
            i = parent;
            parent = (i - 1) / 2;
        }
    }

    fn heapify(&mut self) {
        for i in (0..self.heap.len()).rev() {
            self.sift_down(i);
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let mut left = i * 2 + 1;
        let mut right = i * 2 + 2;
        // While left or right node is larger than current (i) node
        while left < self.heap.len() && self.heap[i] > self.heap[left]
            || right < self.heap.len() && self.heap[i] > self.heap[right]
        {
            let smallest = if right < self.heap.len() {
                if self.heap[left] < self.heap[right] {
                    left
                } else {
                    right
                }
            } else {
                left
            };
            self.heap.swap(i, smallest);
            i = smallest;
            left = i * 2 + 1;
            right = i * 2 + 2;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PriorityQueue;
    #[test]
    fn new_sorts() {
        let mut queue = PriorityQueue::new(vec![3, 2, 6, 5, 1, 4]);
        assert_eq!(queue.take_min(), Some(1));
        assert_eq!(queue.take_min(), Some(2));
        assert_eq!(queue.take_min(), Some(3));
        assert_eq!(queue.take_min(), Some(4));
        assert_eq!(queue.take_min(), Some(5));
        assert_eq!(queue.take_min(), Some(6));
        assert_eq!(queue.take_min(), None);
    }

    #[test]
    fn insert_works() {
        let mut queue = PriorityQueue::new(vec![1, 5, 9]);
        queue.insert(8);
        assert_eq!(queue.take_min(), Some(1));
        assert_eq!(queue.take_min(), Some(5));
        assert_eq!(queue.take_min(), Some(8));
        assert_eq!(queue.take_min(), Some(9));
        assert_eq!(queue.take_min(), None);
    }
}
