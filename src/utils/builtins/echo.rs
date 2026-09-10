use crate::kprint;
use crate::kprintln;

pub fn exec(args: core::str::SplitWhitespace) {
    let mut first = true;

    for arg in args {
        if !first {
            kprint!(" ");
        }
        kprint!("{}", arg);
        first = false;
    }
    kprintln!("");
}
