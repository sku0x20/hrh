static IS_DEBUG_ENABLED: bool = true;
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}
