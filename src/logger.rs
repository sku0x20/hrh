static IS_DEBUG_ENABLED: bool = false;
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}
