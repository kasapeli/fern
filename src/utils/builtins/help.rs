use crate::kprintln;

pub fn exec(args: &mut core::str::SplitWhitespace) {
    match args.next() {
        None => {
            kprintln!("version, help, clear, reboot, panic, halt, echo, ginfo");
            kprintln!("for verbose help, try help <command>");
        }
        Some("version") => {
            kprintln!("version info");
            kprintln!("usage: version");
            kprintln!("displays info about fern and fsh's current version");
        }
        Some("help") => {
            kprintln!("help info");
            kprintln!("usage: (1) help, (2) help <command>");
            kprintln!("(1) shows a list of command");
            kprintln!("(2) shows help for a specific command");
        }
        Some("clear") => {
            kprintln!("clear info");
            kprintln!("usage: clear");
            kprintln!("clears the screen");
        }
        Some("reboot") => {
            kprintln!("reboot info");
            kprintln!("usage: reboot");
            kprintln!("reboots the system");
        }
        Some("panic") => {
            kprintln!("panic info");
            kprintln!("usage: panic");
            kprintln!("makes the system panic");
        }
        Some("halt") => {
            kprintln!("halt info");
            kprintln!("usage: halt");
            kprintln!("halts the CPU");
        }
        Some("echo") => {
            kprintln!("echo info");
            kprintln!("usage: (1) echo, (2) echo <message>");
            kprintln!("(1) echoes a newline containing nothing");
            kprintln!("(2) echoes a message");
        }
        Some("ginfo") => {
            kprintln!("ginfo info");
            kprintln!("usage: ginfo <section>");
            kprintln!("prints info about a hardware section");
        }
        _ => {
            kprintln!(
                "invalid argument, try help for command list, or help <command> for verbose help"
            );
        }
    }
}
