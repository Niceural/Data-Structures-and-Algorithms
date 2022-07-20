pub trait Queue<T> {
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0;
    }
    fn push_back(&mut self, val: T);
    fn pop_front(&mut self) -> Option<T>;
}

pub struct FixedCapacityQueue<T: Clone> {
    array: Box<[Option<T>]>,
    front: usize,
    back: usize,
    capacity: usize,
}

impl<T: Clone> FixedCapacityQueue<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            front: 0,
            back: 0,
            capacity,
            array: vec![None; capacity].into_boxed_slice(),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.array.get(self.front).and_then(|x| x.as_ref())
    }
}

impl<T: Clone> Queue<T> for FixedCapacityQueue<T> {
    fn len(&self) -> usize {
        self.back - self.front
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn push_back(&mut self, val: T) {
        assert!(self.back < self.capacity, "Queue too small");
        self.array[self.back] = Some(val);
        self.back += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let res = self.array[self.front].take();
            self.front += 1;
            res
        }
    }
}