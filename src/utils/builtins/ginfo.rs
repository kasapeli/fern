extern crate alloc;

use crate::kprintln;
use crate::utils::builtins::*;

pub fn exec(args: &[&str]) {
    if args.is_empty() {
        kprintln!("no argument provided, try ginfo help");
        return;
    }

    match args[0] {
        "help" => help(),
        "cpu" => cpuinfo::exec(),
        "heap" => heapinfo::exec(),
        _ => {
            kprintln!("invalid argument, try ginfo help");
        }
    }
}

fn help() {
    kprintln!("ginfo a0.1");
    kprintln!("-- arguments --");
    kprintln!("{:<5} {}", "help", "shows this text");
    kprintln!("{:<5} {}", "cpu", "shows cpu info");
    kprintln!("{:<5} {}", "heap", "shows heap size");
}
