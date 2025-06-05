//! Passing vectors from C++ to Rust
//! 
//! 
//!

/// Struct: Vector<T>
/// 
/// ## Description
/// 
/// 
/// C/C++ compatible f64 vectors
/// 
/// NOTE: if bindgen is C++ this is a struct template
/// otherwise its a typedef (For some reason it becomes vector_f64 if exported like this)
#[repr(C)]
pub struct vector<T> {
    pub data: *const T,
    pub len:  usize,
    pub idx:  usize,
}

impl<T> vector<T> {
    
    pub fn new(data: *const T, len: usize, idx: usize) -> Self {
        vector { data, len, idx }
    }

    pub fn as_vector(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.data as *mut T, self.len)

        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sum(vector: *const vector<f64>) -> f64 {

    // convert to to reference
    let reference = unsafe {
        &*vector
    };

    reference.as_vector().iter().sum()

}


mod unit_tests {
    use super::{vector, sum};
    use std::mem;

    #[test]
    fn is_sum_correct() {

        let mut original = vec![1.3, 2.4];
        let len = original.len();
        let cap = original.capacity();

        let ptr: *const f64 = original.as_mut_ptr();

        let raw = vector::new(ptr, len, cap);
        let result: f64 = unsafe { sum(&raw) };

        assert_eq!(3.7, result);
    }
}
