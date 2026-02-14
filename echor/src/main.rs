use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// The string to echo
    #[arg()]
    input: String,
    
    #[arg(short = 'n', help = "Do not print the trailing newline")]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();

    if args.no_newline {
        print!("{}", args.input);
    } else {
        println!("{}", args.input);
    }
}