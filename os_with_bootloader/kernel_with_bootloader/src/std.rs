pub(crate) mod prelude;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::FRAME_BUFFER_WRITER.lock(), "{}", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::FRAME_BUFFER_WRITER.lock(), "{}", format_args_nl!($($arg)*)).unwrap();
    }};
}