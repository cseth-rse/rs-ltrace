mod tracer;
mod ptrace;
mod registers;
mod error;

use tracer::Tracer;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  rs-ltrace -- <program> [args...]");
        return;
    }

    // Remove program name
    args.remove(0);

    if args[0] != "--" {
        eprintln!("You must use '--' before program name.");
        eprintln!("Example: rs-ltrace -- sleep 5");
        return;
    }

    args.remove(0); // remove "--"

    if args.is_empty() {
        eprintln!("No program specified.");
        return;
    }

    let mut tracer = Tracer::new(args);

    if let Err(e) = tracer.run() {
        eprintln!("Error: {}", e);
    }
}
