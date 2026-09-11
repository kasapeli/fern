#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod drivers;
mod flib;
mod utils;

use crate::flib::kprint;

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        asm!("cli", "hlt", options(nomem, nostack));
    }

    kprint::set_color(0x1F);
    kprint::clear_screen();

    kprintln!("!!! KERNEL PANIC !!!");
    kprintln!("{}", info);

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kprintln!(
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
    kprintln!(
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
