use std::collections::HashMap;
use std::cmp;
use std::hash::Hash;
use std::cmp::Ordering;

pub struct Counter<T> {
    counter: HashMap<T, u32>,
}

impl<T> Counter<T>
where
    T: cmp::Ord,
    T: cmp::Eq,
    T: Hash,
    T: Copy,
{
    pub fn new() -> Counter<T> {
        Counter {
            counter: HashMap::new(),
        }
    }
    pub fn add(&mut self, item: &T, count: u32) {
        *self.counter.entry(*item).or_insert(0) += count;
    }
    pub fn get(&self, item: &T) -> u32 {
        match self.counter.get(&item) {
            Some(count) => *count,
            None => 0,
        }
    }
    pub fn items_with_count_at_least(&self, min_count: u32) -> Vec<T> {
        self.counter
            .keys()
            .cloned()
            .filter(|item| self.get(item) > min_count)
            .collect()
    }
    pub fn sort_descending(&self, v: &mut Vec<T>) {
        v.sort_by(|a, b| {
            let count_a = self.get(a);
            let count_b = self.get(b);
            if count_a == count_b {
                return b.cmp(a);
            }
            count_b.cmp(&count_a)
        });
    }
}
