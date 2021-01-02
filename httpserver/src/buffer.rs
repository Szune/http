use std::collections::VecDeque;
use std::iter::{Iterator};

pub struct Buffer<I: Iterator> {
    values: VecDeque<Option<I::Item>>,
    iter: I,
}

impl<I: Iterator> Buffer<I> {
    pub fn peek(&self, depth: usize) -> &Option<I::Item> {
        self.values.get(depth).unwrap_or(&None)
    }
}

impl<I: Iterator> Iterator for Buffer<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // enqueue next value
        let next_value = self.iter.next();
        self.values.push_back(next_value);
        // dequeue first value
        self.values.pop_front().unwrap_or(None)
    }
}

pub trait Buffered : Iterator {
    fn buffer(mut self, buffer_size: usize) -> Buffer<Self>
        where Self: Sized
    {
        // setup initial values for peeking
        let mut values = VecDeque::<Option<Self::Item>>::new();
        for _ in 0..buffer_size {
            values.push_back(self.next());
        }
        let buffer = Buffer {
            values,
            iter: self,
        };
        buffer
    }
}

impl<I: Iterator> Buffered for I { }

