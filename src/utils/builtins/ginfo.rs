use crate::kprintln;

pub fn exec(args: &mut core::str::SplitWhitespace) {
    match args.next() {
        Some("cpu") => crate::utils::builtins::cpuinfo::exec(),
        Some("help") => help(),
        None => {
            kprintln!("no arguments provided, try ginfo help");
        }
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
}
