#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod drivers;
mod flib;
mod utils;

use flib::vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("!!! KERNEL PANIC !!!");
    println!("{}", info);
    loop {}
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
