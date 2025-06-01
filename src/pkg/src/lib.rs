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

