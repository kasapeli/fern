unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}
pub fn get_scancode() -> u8 {
    unsafe {
        if (inb(0x64) & 0x01) != 0 {
            return inb(0x60);
        }
    }
    0
}

pub fn scancode_to_char(scancode: u8) -> char {
    match scancode {
        0x02 => '1',
        0x03 => '2',
        0x04 => '3',
        0x05 => '4',
        0x06 => '5',
        0x07 => '6',
        0x08 => '7',
        0x09 => '8',
        0x0A => '9',
        0x0B => '0',
        0x10 => 'q',
        0x11 => 'w',
        0x12 => 'e',
        0x13 => 'r',
        0x14 => 't',
        0x15 => 'y',
        0x16 => 'u',
        0x17 => 'i',
        0x18 => 'o',
        0x19 => 'p',
        0x1E => 'a',
        0x1F => 's',
        0x20 => 'd',
        0x21 => 'f',
        0x22 => 'g',
        0x23 => 'h',
        0x24 => 'j',
        0x25 => 'k',
        0x26 => 'l',
        0x2C => 'z',
        0x2D => 'x',
        0x2E => 'c',
        0x2F => 'v',
        0x30 => 'b',
        0x31 => 'n',
        0x32 => 'm',
        0x39 => ' ',
        0x1C => '\n',
        _ => '\0',
    }
}

pub fn read_char() -> char {
    loop {
        let scancode = get_scancode();
        if scancode != 0 {
            let c = scancode_to_char(scancode);
            if c != '\0' {
                return c;
            }
        }
    }
}
