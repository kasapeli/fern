use crate::print;
use crate::println;

pub fn exec(args: core::str::SplitWhitespace) {
    let mut first = true;

    for arg in args {
        if !first {
            print!(" ");
        }
        print!("{}", arg);
        first = false;
    }
    println!("");
}
