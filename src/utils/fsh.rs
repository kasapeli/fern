extern crate alloc;

use crate::drivers::keyboard;
use crate::utils::builtins;
use crate::{kprint, kprintln};
use alloc::string::String;
use alloc::vec::Vec;

pub fn fsh() {
    kprint!("fsh> ");

    let mut cmd = String::new();

    loop {
        let c = keyboard::read_char();

        if c == '\n' {
            kprintln!("");

            if !cmd.is_empty() {
                let tokens: Vec<&str> = cmd.split_whitespace().collect();
                if !tokens.is_empty() {
                    exec(&tokens);
                }
            }

            cmd.clear();
            kprint!("fsh> ");
        } else if c == '\x08' {
            if !cmd.is_empty() {
                cmd.pop();
                kprint::backspace();
            }
        } else {
            kprint!("{}", c);
            cmd.push(c);
        }
    }
}

pub fn exec(tokens: &[&str]) {
    let command = tokens[0];
    let args = &tokens[1..];

    match command {
        "help" => builtins::help::exec(args),
        "version" => builtins::version::exec(),
        "panic" => builtins::panic::exec(),
        "clear" => builtins::clear::exec(),
        "halt" => builtins::halt::exec(),
        "ginfo" => builtins::ginfo::exec(args),
        "reboot" => builtins::reboot::exec(),
        "echo" => builtins::echo::exec(args),
        _ => {
            kprintln!("fsh: invalid command: {}", command);
        }
    }
}
