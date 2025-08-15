pub static mut IS_DEBUG_ENABLED: bool = false;
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        unsafe {
            if crate::logger::IS_DEBUG_ENABLED {
                println!($($arg)*);
            }
        }
    }};
}

pub fn init_logger() {
    let is_enabled = std::env::var("DEBUG").unwrap_or(String::from("0"));
    let is_enabled = if is_enabled == "1" { true } else { false };
    unsafe {
        IS_DEBUG_ENABLED = is_enabled;
    }
}
