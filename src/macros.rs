#[macro_export]
macro_rules! print_flush {
    ($($arg:tt)*) => {{
        use std::io::{Write, stdout};
        print!($($arg)*);
        stdout().flush().expect("Failed to flush stdout");
    }};
}
