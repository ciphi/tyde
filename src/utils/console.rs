/// Format the text to a green color.
#[macro_export]
macro_rules! positive {
    ($($arg:tt)*) => {
        ::owo_colors::OwoColorize::green(&format!($($arg)*))
    };
}

/// Prints to the terminal with a green color.
#[macro_export]
macro_rules! positiveln {
    ($($arg:tt)*) => {
        println!("{}", $crate::positive!($($arg)*));
    };
}

/// Format the text to a red color.
#[macro_export]
macro_rules! negative {
    ($($arg:tt)*) => {
        ::owo_colors::OwoColorize::red(&format!($($arg)*))
    };
}

/// Prints to the terminal with a red color.
#[macro_export]
macro_rules! negativeln {
    ($($arg:tt)*) => {
        println!("{}", $crate::negative!($($arg)*));
    };
}

/// Format the text to a blue color.
#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => {
        ::owo_colors::OwoColorize::cyan(&format!($($arg)*))
    };
}

/// Prints to the terminal with a blue color.
#[macro_export]
macro_rules! noticeln {
    ($($arg:tt)*) => {
        println!("{}", $crate::notice!($($arg)*));
    };
}
