/// C/C++ compatible f64 vectors
#[repr(C)]
pub struct F64Slice {
    pub data: *const f64,
    pub len:  usize,
    pub idx: usize
}

impl F64Slice {

    pub fn new(data: *const f64, len: usize) -> Self {
        F64Slice { data, len, idx: 0 }
    }

    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

impl Iterator for F64Slice {
    type Item = f64;

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

/// With F64Slice implementations
#[unsafe(no_mangle)]
pub extern "C" fn sum_slice(slice: *const F64Slice) -> f64 {
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

