use crate::kprintln;

pub fn exec(args: &[&str]) {
    if args.is_empty() {
        kprintln!("version, help, clear, reboot, panic, halt, echo, ginfo");
        kprintln!("for verbose help, try help <command>");
        return;
    }

    match args[0] {
        "version" => {
            kprintln!("version info");
            kprintln!("usage: version");
            kprintln!("displays info about fern and fsh's current version");
        }
        "help" => {
            kprintln!("help info");
            kprintln!("usage: (1) help, (2) help <command>");
            kprintln!("(1) shows a list of command");
            kprintln!("(2) shows help for a specific command");
        }
        "clear" => {
            kprintln!("clear info");
            kprintln!("usage: clear");
            kprintln!("clears the screen");
        }
        "reboot" => {
            kprintln!("reboot info");
            kprintln!("usage: reboot");
            kprintln!("reboots the system");
        }
        "panic" => {
            kprintln!("panic info");
            kprintln!("usage: panic");
            kprintln!("makes the system panic");
        }
        "halt" => {
            kprintln!("halt info");
            kprintln!("usage: halt");
            kprintln!("halts the CPU");
        }
        "echo" => {
            kprintln!("echo info");
            kprintln!("usage: (1) echo, (2) echo <message>");
            kprintln!("(1) echoes a newline containing nothing");
            kprintln!("(2) echoes a message");
        }
        "ginfo" => {
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
