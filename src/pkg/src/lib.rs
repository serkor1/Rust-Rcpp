/// C/C++ compatible f64 vectors
#[repr(C)]
pub struct RawSlice<T> {
    pub data: *const T,
    pub len:  usize,
    pub idx: usize
}

impl<T: Copy> RawSlice<T> {

    pub fn new(data: *const T, len: usize) -> Self {
        RawSlice { data, len, idx: 0 }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

impl<T: Copy> Iterator for RawSlice<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let value = unsafe { *self.data.add(self.idx) };
            self.idx += 1;
            Some(value)
        }
    }
}

/// With RawSlice implementations
#[unsafe(no_mangle)]
pub extern "C" fn sum_slice(slice: *const RawSlice<f64>) -> f64 {
    let slice_ref = unsafe { &*slice };
    slice_ref.as_slice().iter().sum()
}

/// add two values
#[unsafe(no_mangle)]
pub extern "C" fn add(left: f64, right: f64) -> f64 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn sum(data: *const f64, len: usize) -> f64 {
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    slice.iter().sum()
}


#[cfg(test)]
mod unit_tests {
    use super::*;
    static VECTOR: [f64; 2] = [1.3, 2.4]; // Sums to 3.7 incase you wanted to know ¯\_(ツ)_/¯

    #[test]
    fn is_sum_correct() {
        // This test is mostly conducted
        // as a guide on how to use the struct
        // on the rust side.
        let ptr: *const f64 = VECTOR.as_ptr();
        let len: usize = VECTOR.len();

        let raw = RawSlice::new(ptr, len);
        let with_struct: f64 = unsafe { sum_slice(&raw as *const RawSlice<f64>) };
        let without_struct: f64 = unsafe { sum(ptr, len) };

        assert_eq!(with_struct, without_struct);
        assert_eq!(3.7, with_struct);
    }


}
