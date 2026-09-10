#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod drivers;
mod flib;
mod utils;

use flib::vga;

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack));
    }

    unsafe {
        crate::vga::WRITER.set_color(0x1F);
        crate::vga::WRITER.clear_screen();
    }

    println!("!!! KERNEL PANIC !!!");
    println!("{}", info);
    println!("q to reboot");

    loop {
        unsafe {
            let mut status: u8;
            let mut scancode: u8;

            asm!("in al, 0x64", out("al") status, options(nomem, nostack));
            if (status & 0x01) != 0 {
                asm!("in al, 0x60", out("al") scancode, options(nomem, nostack));

                if scancode == 0x10 {
                    crate::utils::builtins::reboot::exec();
                }
            }
            core::hint::spin_loop();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!(
        "
   :####                                
   #####                                
   ##                                   
 #######    .####:    ##.####  ##.####  
 #######   .######:   #######  #######  
   ##      ##:  :##   ###.     ###  :## 
   ##      ########   ##       ##    ## 
   ##      ########   ##       ##    ## 
   ##      ##         ##       ##    ## 
   ##      ###.  :#   ##       ##    ## 
   ##      .#######   ##       ##    ## 
   ##       .#####:   ##       ##    ##
__________________________________________"
    );
    utils::builtins::version::exec();
    println!(
        "
Welcome to Fern!
    "
    );
    loop {
        utils::fsh::fsh();

        unsafe {
            core::arch::asm!("sti", "hlt", options(nomem, nostack, preserves_flags));
        }
    }
}
