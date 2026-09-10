use core::arch::asm;
use core::fmt;
use core::ptr::{read_volatile, write_volatile};

pub struct VgaWriter {
    cursor: usize,
    vga_addr: usize,
    color_code: u8,
}

impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            cursor: 0,
            vga_addr: 0xB8000,
            color_code: 0x0F,
        }
    }

    pub fn set_color(&mut self, color: u8) {
        self.color_code = color;
    }

    #[inline]
    unsafe fn outb(port: u16, val: u8) {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, nostack, preserves_flags)
        );
    }

    pub fn update_hardware_cursor(&self) {
        unsafe {
            Self::outb(0x3D4, 0x0F);
            Self::outb(0x3D5, (self.cursor & 0xFF) as u8);

            Self::outb(0x3D4, 0x0E);
            Self::outb(0x3D5, ((self.cursor >> 8) & 0xFF) as u8);
        }
    }

    pub fn clear_screen(&mut self) {
        const TOTAL: usize = 80 * 25;

        unsafe {
            let base_ptr = self.vga_addr as *mut u8;
            for i in 0..TOTAL {
                let cell_offset = i * 2;
                write_volatile(base_ptr.add(cell_offset), b' ');
                write_volatile(base_ptr.add(cell_offset + 1), self.color_code);
            }
        }
        self.cursor = 0;
        self.update_hardware_cursor();
    }

    pub fn scroll(&mut self) {
        unsafe {
            let base_ptr = self.vga_addr as *mut u8;

            for row in 1..25 {
                for col in 0..80 {
                    let dest_offset = ((row - 1) * 80 + col) * 2;
                    let src_offset = (row * 80 + col) * 2;

                    let char_byte = read_volatile(base_ptr.add(src_offset));
                    let attr_byte = read_volatile(base_ptr.add(src_offset + 1));

                    write_volatile(base_ptr.add(dest_offset), char_byte);
                    write_volatile(base_ptr.add(dest_offset + 1), attr_byte);
                }
            }

            for col in 0..80 {
                let offset = (24 * 80 + col) * 2;
                write_volatile(base_ptr.add(offset), b' ');
                write_volatile(base_ptr.add(offset + 1), self.color_code);
            }
        }

        self.cursor = 24 * 80;
    }

    pub fn putchar(&mut self, char: u8) {
        if self.cursor >= 80 * 25 {
            self.scroll();
        }

        if char == b'\n' {
            let row = self.cursor / 80;
            self.cursor = (row + 1) * 80;

            if self.cursor >= 80 * 25 {
                self.scroll();
            }
            return;
        }

        let ptr = (self.vga_addr + self.cursor * 2) as *mut u8;
        unsafe {
            write_volatile(ptr, char);
            write_volatile(ptr.add(1), self.color_code);
        }

        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let ptr = (self.vga_addr + self.cursor * 2) as *mut u8;
            unsafe {
                write_volatile(ptr, b' ');
                write_volatile(ptr.add(1), self.color_code);
            }
            self.update_hardware_cursor();
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            self.putchar(byte);
        }
        self.update_hardware_cursor();
        Ok(())
    }
}

pub static mut WRITER: VgaWriter = VgaWriter::new();

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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            let _ = core::write!($crate::vga::WRITER, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
