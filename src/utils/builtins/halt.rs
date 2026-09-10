use crate::println;

pub fn exec() {
    println!("halting");
    unsafe {
        core::arch::asm!("cli; hlt");
    }
}
