use crate::drivers::keyboard;
use crate::print;
use crate::println;
use crate::utils::builtins;
use crate::vga;

pub fn fsh() {
    print!("fsh> ");

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
            print!("fsh> ");
        } else if c == '\x08' {
            if index > 0 {
                index -= 1;
                vga::backspace();
            }
        } else if index < cmdbuf.len() {
            print!("{}", c);
            cmdbuf[index] = c as u8;
            index += 1;
        }
    }
}

pub fn exec(cmd: &str) {
    let mut parts = cmd.split_whitespace();

    let command = match parts.next() {
        Some(c) => c,
        None => return,
    };

    match command {
        "help" => builtins::help::exec(),
        "version" => builtins::version::exec(),
        "panic" => builtins::panic::exec(),
        "clear" => builtins::clear::exec(),
        "halt" => builtins::halt::exec(),
        "ginfo" => builtins::ginfo::exec(&mut parts),
        "reboot" => builtins::reboot::exec(),
        "echo" => builtins::echo::exec(parts),
        _ => {
            println!("fsh: invalid command: {}", command);
        }
    }
}
