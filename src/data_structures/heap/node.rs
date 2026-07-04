/// This is a node in the heap. 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HeapNode {
    /// The identity of a value in the heap.
    pub value_identity: i32,
    /// The frequency a given value occurs in the heap.
    pub frequency: i32,
}


impl HeapNode {
    
    /// Creates a new `HeapNode`.
    pub fn new(value_identity: i32, frequency: i32) -> Self {
        HeapNode {
            value_identity,
            frequency,
        }
    }
}