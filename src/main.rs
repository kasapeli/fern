#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod drivers;
mod flib;

use drivers::keyboard;
use flib::println;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC!");
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Initializing...");
    println!("Welcome to Fern!");
    print!("fsh>");

    let mut cmdbuf = [0u8; 64];
    let mut index = 0;

    loop {
        let c = keyboard::read_char();
        if c == '\n' {
            println!("");

            if index > 0 {
                if let Ok(cmd) = core::str::from_utf8(&cmdbuf[..index]) {
                    exec(cmd);
                }
            }

            index = 0;
            print!("fsh>");
        } else if c == '\x08' {
            if index > 0 {
                index -= 1;
                println::backspace();
            }
        } else if index < cmdbuf.len() {
            print!("{}", c);
            cmdbuf[index] = c as u8;
            index += 1;
        }
    }
}

fn exec(cmd: &str) {
    match cmd {
        "help" => {
            println!("version, help, halt");
        }
        "version" => {
            println!("fern a0.1");
            println!("fern shell a0.1");
        }
        "halt" => {
            println!("halting");
            unsafe {
                core::arch::asm!("cli; hlt");
            }
        }
        _ => println!("invalid command: {}.", cmd),
    }
}
