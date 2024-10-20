// /*
// 	heap
// 	This question requires you to implement a binary heap function
// */


// use std::cmp::Ord;
// use std::default::Default;

// pub struct Heap<T>
// where
//     T: Default,
// {
//     count: usize,
//     items: Vec<T>,
//     comparator: fn(&T, &T) -> bool,
// }

// impl<T> Heap<T>
// where
//     T: Default,
// {
//     pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
//         Self {
//             count: 0,
//             items: vec![T::default()],
//             comparator,
//         }
//     }

//     pub fn len(&self) -> usize {
//         self.count
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     pub fn add(&mut self, value: T) {
//         //TODO
//     }

//     fn parent_idx(&self, idx: usize) -> usize {
//         idx / 2
//     }

//     fn children_present(&self, idx: usize) -> bool {
//         self.left_child_idx(idx) <= self.count
//     }

//     fn left_child_idx(&self, idx: usize) -> usize {
//         idx * 2
//     }

//     fn right_child_idx(&self, idx: usize) -> usize {
//         self.left_child_idx(idx) + 1
//     }

//     fn smallest_child_idx(&self, idx: usize) -> usize {
//         //TODO
// 		0
//     }
// }

// impl<T> Heap<T>
// where
//     T: Default + Ord,
// {
//     /// Create a new MinHeap
//     pub fn new_min() -> Self {
//         Self::new(|a, b| a < b)
//     }

//     /// Create a new MaxHeap
//     pub fn new_max() -> Self {
//         Self::new(|a, b| a > b)
//     }
// }

// impl<T> Iterator for Heap<T>
// where
//     T: Default,
// {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         //TODO
// 		None
//     }
// }

// pub struct MinHeap;

// impl MinHeap {
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new<T>() -> Heap<T>
//     where
//         T: Default + Ord,
//     {
//         Heap::new(|a, b| a < b)
//     }
// }

// pub struct MaxHeap;

// impl MaxHeap {
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new<T>() -> Heap<T>
//     where
//         T: Default + Ord,
//     {
//         Heap::new(|a, b| a > b)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_empty_heap() {
//         let mut heap = MaxHeap::new::<i32>();
//         assert_eq!(heap.next(), None);
//     }

//     #[test]
//     fn test_min_heap() {
//         let mut heap = MinHeap::new();
//         heap.add(4);
//         heap.add(2);
//         heap.add(9);
//         heap.add(11);
//         assert_eq!(heap.len(), 4);
//         assert_eq!(heap.next(), Some(2));
//         assert_eq!(heap.next(), Some(4));
//         assert_eq!(heap.next(), Some(9));
//         heap.add(1);
//         assert_eq!(heap.next(), Some(1));
//     }

//     #[test]
//     fn test_max_heap() {
//         let mut heap = MaxHeap::new();
//         heap.add(4);
//         heap.add(2);
//         heap.add(9);
//         heap.add(11);
//         assert_eq!(heap.len(), 4);
//         assert_eq!(heap.next(), Some(11));
//         assert_eq!(heap.next(), Some(9));
//         assert_eq!(heap.next(), Some(4));
//         heap.add(1);
//         assert_eq!(heap.next(), Some(2));
//     }
// }


/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // 元素首先入队到q2
        self.q2.enqueue(elem);

        // 将q1中的所有元素转移到q2
        while !self.q1.is_empty() {
            if let Ok(data) = self.q1.dequeue() {
                self.q2.enqueue(data);
            }
        }

        // 交换q1和q2
        std::mem::swap(&mut self.q1, &mut self.q2);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
        self.q1.dequeue()
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}