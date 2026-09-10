use core::fmt;

pub struct VgaWriter {
    cursor: usize,
    vga_addr: usize,
}

impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            cursor: 0,
            vga_addr: 0xB8000,
        }
    }

    pub fn putchar(&mut self, char: u8) {
        let ptr = (self.vga_addr + self.cursor * 2) as *mut u8;

        if char == b'\n' {
            let row = self.cursor / 80;
            self.cursor = (row + 1) * 80;

            if self.cursor >= 80 * 25 {
                self.cursor = 0;
            }

            return;
        }

        unsafe {
            *ptr = char;
            *ptr.add(1) = 0x0F;
        }

        self.cursor += 1;
    }

    // why's this in println
    // consider moving to the keyboard driver smth instead later
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let ptr = (self.vga_addr + self.cursor * 2) as *mut u8;
            unsafe {
                *ptr = b' ';
                *ptr.add(1) = 0x0F;
            }
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            self.putchar(byte);
        }
        Ok(())
    }
}

pub static mut WRITER: VgaWriter = VgaWriter::new();

pub fn backspace() {
    unsafe {
        WRITER.backspace();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            let _ = core::write!($crate::println::WRITER, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
