use std::collections::HashMap;
use serde::{Serialize, Deserialize};
// use crate::core::error::VMError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeapValue {
    Array(Vec<i64>),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapManager {
    heap: HashMap<usize, HeapValue>,
    next_id: usize,
}

impl HeapManager {
    pub fn new() -> Self {
        Self {
            heap: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn allocate(&mut self, value: HeapValue) -> usize {
        let id = self.next_id;
        self.heap.insert(id, value);
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: usize) -> Option<&HeapValue> {
        self.heap.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut HeapValue> {
        self.heap.get_mut(&id)
    }

    pub fn free(&mut self, id: usize) -> Option<HeapValue> {
        self.heap.remove(&id)
    }

    pub fn is_valid_address(&self, id: usize) -> bool {
        self.heap.contains_key(&id)
    }
}