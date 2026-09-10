use crate::drivers::vga::WRITER;
use core::fmt::Write;

pub fn clear_screen() {
    unsafe {
        WRITER.clear_screen();
    }
}

pub fn backspace() {
    unsafe {
        WRITER.backspace();
    }
}

pub fn set_color(color: u8) {
    unsafe {
        WRITER.set_color(color);
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            let _ = core::write!($crate::drivers::vga::WRITER, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}
