use crate::println;

pub fn exec(args: &mut core::str::SplitWhitespace) {
    match args.next() {
        Some("cpu") => crate::utils::builtins::cpuinfo::exec(),
        Some("help") => help(),
        None => {
            println!("no arguments provided, try ginfo help");
        }
        _ => {
            println!("invalid argument, try ginfo help");
        }
    }
}

fn help() {
    println!("ginfo a0.1");
    println!("-- arguments --");
    println!("{:<5} {}", "help", "shows this text");
    println!("{:<5} {}", "cpu", "shows cpu info");
}
