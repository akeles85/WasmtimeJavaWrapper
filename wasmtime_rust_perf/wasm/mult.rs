#[no_mangle]
pub extern "C" fn multiply(n1: f32, n2: f32) -> f32 {
    let mut result = 0.0;
    for n in 0..10_000_000 {
        result += n1.cos()*n2.sin();
    }
    return result;
}