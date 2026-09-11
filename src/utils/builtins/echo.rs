use crate::kprintln;

pub fn exec(args: &[&str]) {
    if args.is_empty() {
        kprintln!("");
        return;
    }

    let o = args.join("");

    kprintln!("{}", o);
}
