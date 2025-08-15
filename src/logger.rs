pub static IS_DEBUG_ENABLED: bool = false;
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if crate::logger::IS_DEBUG_ENABLED {
            println!($($arg)*);
        }
    }};
}
