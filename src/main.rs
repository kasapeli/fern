#![no_std]
#![no_main]
#![allow(static_mut_refs)]

static mut HEAP_MEM: [u8; 256 * 1024] = [0; 256 * 1024];

#[global_allocator]
static ALLOCATOR: flib::kmalloc::Allocator = flib::kmalloc::Allocator::new();

mod drivers;
mod flib;
mod utils;

extern crate alloc;
use alloc::vec::Vec;

use crate::flib::kprint;

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprint::set_color(0x1F);
    kprint::clear_screen();

    kprintln!("!!! KERNEL PANIC !!!");
    kprintln!("{}", info);

    loop {
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        let heap_start = HEAP_MEM.as_ptr() as usize;
        let heap_size = HEAP_MEM.len();
        ALLOCATOR.init(heap_start, heap_size);
    }

    let mut v: Vec<u8> = alloc::vec::Vec::new();
    v.push(42);
    kprintln!("heap: OK");

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
