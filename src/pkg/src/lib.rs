/// C/C++ compatible f64 vectors
/// NOTE: Could be done with traits
/// too
#[repr(C)]
pub struct F64Slice {
    pub data: *const f64,
    pub len:  usize,
}

impl F64Slice {
    pub fn as_slice(&self) -> &[f64] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

/// With F64Slice implementations
#[unsafe(no_mangle)]
pub extern "C" fn sum_slice(slice: *const F64Slice) -> f64 {
    let slice_ref = unsafe { &*slice };
    slice_ref.as_slice().into_iter().sum()
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

