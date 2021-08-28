#[macro_export]
macro_rules! format_error {
    ($($args:tt)*) => {{
        format!("[ERROR] {}", format_args!($($args)*))
    }};
}

#[macro_export]
macro_rules! format_warning {
    ($($args:tt)*) => {{
        format!("[WARN] {}", format_args!($($args)*))
    }};
}

#[macro_export]
macro_rules! format_info {
    ($($args:tt)*) => {{
        format!("[INFO] {}", format_args!($($args)*))
    }};
}

#[macro_export]
macro_rules! print_error {
    ($($args:tt)*) => {{
        println!("[ERROR] {}", format_args!($($args)*))
    }};
}

#[macro_export]
macro_rules! print_warning {
    ($($args:tt)*) => {{
        println!("[WARN] {}", format_args!($($args)*))
    }};
}

#[macro_export]
macro_rules! print_info {
    ($($args:tt)*) => {{
        println!("[INFO] {}", format_args!($($args)*))
    }};
}