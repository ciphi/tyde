use std::sync::atomic::{AtomicBool, Ordering};

pub static VERBOSE_OUTPUT: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(enabled: bool) {
    VERBOSE_OUTPUT.store(enabled, Ordering::Relaxed);
    if enabled {
        use owo_colors::OwoColorize;
        println!("{}", "Verbosity enabled".bright_cyan());
    }
}

pub fn is_verbose() -> bool {
    VERBOSE_OUTPUT.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! verbose {
    ($msg:expr) => {
        if $crate::utils::verbose::is_verbose() {
            use owo_colors::OwoColorize;
            println!("{}", $msg.bright_cyan());
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        if $crate::utils::verbose::is_verbose() {
            use owo_colors::OwoColorize;
            let formatted = format!($fmt, $($arg)*);
            println!("{}", formatted.bright_cyan());
        }
    };
}
