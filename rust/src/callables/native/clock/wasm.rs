extern "C" {
    fn get_current_js_time() -> u32;
}

pub fn get_current_time() -> u64 {
    unsafe { get_current_js_time() as u64 }
}
