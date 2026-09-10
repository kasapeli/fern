use crate::kprintln;

pub fn exec() {
    kprintln!("halting");
    unsafe {
        core::arch::asm!("cli; hlt");
    }
}
