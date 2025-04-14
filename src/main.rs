use clap::Parser;
use std::io::Write;
use typeflow::CharFlow;

#[derive(Parser, Debug)]
struct Args {
    message: String,
    #[arg(short, long, default_value_t = 50)]
    delay: u64,
}

fn main() {
    let args = Args::parse();
    let flow = CharFlow::new(&args.message, args.delay);

    for ch in flow {
        print!("{}", ch);
        std::io::stdout().flush().unwrap();
    }
    println!();
}

