/// This is a node in the heap. 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HeapNode {
    pub value: i32,
}


impl HeapNode {
    
    /// Creates a new `HeapNode`.
    pub fn new(value: i32) -> Self {
        HeapNode {
            value,
        }
    }
}