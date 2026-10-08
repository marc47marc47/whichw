use std::env;

fn main() {
    // Get command-line arguments, skipping the program name
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        eprintln!("Usage: whichw [options] command...");
        eprintln!("       whichw ls");
        eprintln!("       whichw openssl");
        eprintln!("       whichw opewnssl.exe");
        eprintln!("       whichw code.exe");
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  -h, --help    Display this help message");
        std::process::exit(1);
    }

    // Call the which function from lib.rs
    whichw::which(&args);
}

