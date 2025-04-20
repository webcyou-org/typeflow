use clap::Parser;
use std::io::Write;
use typeflow::CharFlow;

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    messages: Vec<String>,
    #[arg(short, long, default_value_t = 50)]
    delay: u64,
}

fn main() {
    let args = Args::parse();

    if args.messages.len() == 1 {
        let flow = CharFlow::new(&args.messages.first().unwrap(), args.delay);
        for ch in flow {
            print!("{}", ch);
            std::io::stdout().flush().unwrap();
        }
        println!();
    } else {
        for message in args.messages {
            let flow = CharFlow::new(&message, args.delay);

            for ch in flow {
                print!("{}", ch);
                std::io::stdout().flush().unwrap();
            }

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }
}
