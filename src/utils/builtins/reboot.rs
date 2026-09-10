use crate::kprintln;
use core::arch::asm;

pub fn exec() -> ! {
    kprintln!("see you again!");
    unsafe {
        loop {
            let mut status: u8;
            asm!("in al, 0x64", out("al") status, options(nomem, nostack));
            if (status & 0x02) == 0 {
                break;
            }
        }

        asm!("out 0x64, al", in("al") 0xFEu8, options(nomem, nostack));
    }

    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
