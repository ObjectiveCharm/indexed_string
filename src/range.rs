
use std::ops::{Add, Range};


/// Helper trait to modify collection of items with range 
pub trait RangeReplaceable where Self::Index: Add<Output=Self::Index> + Copy {
    type Item;

    type Index;
    
    /// Get items in specified range, return None when range out of bounds
    fn items_at_range(&self, range: Range<Self::Index>) -> Option<&Self>;

    /// Replace items in self after parameter `index` with parameter `items`, return `true` when success and `false` when index out of bounds
    fn replace_items(&mut self, index: Self::Index, items: &Self) -> bool;

    /// Get `length` items from parameter `index` 
    fn items(&self, start: Self::Index, length: Self::Index) -> Option<&Self> {
        let range = start..start+length;
        self.items_at_range(range)
    }

}


impl<T> RangeReplaceable for [T] where T: Copy {
    type Item = T;
    type Index = usize;

    fn items_at_range(&self, range: Range<Self::Index>) -> Option<&Self> {
        if range.end <= self.len() {
            Some(&self[range])
        } else {
            None
        }
    }

    fn replace_items(&mut self, start_index: Self::Index, items: &Self) -> bool {
        if start_index + items.len() <= self.len() {
            for (index, &item) in items.iter().enumerate() {
                let replace_index = start_index + index;
                self[replace_index] = item;
            }
            true
        } else {
            false
        }
    }
}