use crate::drivers::keyboard;
use crate::utils::builtins;
use crate::{kprint, kprintln};

pub fn fsh() {
    kprint!("fsh> ");

    let mut cmdbuf = [0u8; 64];
    let mut index = 0;

    loop {
        let c = keyboard::read_char();
        if c == '\n' {
            kprintln!("");

            if index > 0 {
                if let Ok(cmd) = core::str::from_utf8(&cmdbuf[..index]) {
                    exec(cmd);
                }
            }

            index = 0;
            kprint!("fsh> ");
        } else if c == '\x08' {
            if index > 0 {
                index -= 1;
                kprint::backspace();
            }
        } else if index < cmdbuf.len() {
            kprint!("{}", c);
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
        "help" => builtins::help::exec(&mut parts),
        "version" => builtins::version::exec(),
        "panic" => builtins::panic::exec(),
        "clear" => builtins::clear::exec(),
        "halt" => builtins::halt::exec(),
        "ginfo" => builtins::ginfo::exec(&mut parts),
        "reboot" => builtins::reboot::exec(),
        "echo" => builtins::echo::exec(parts),
        _ => {
            kprintln!("fsh: invalid command: {}", command);
        }
    }
}
