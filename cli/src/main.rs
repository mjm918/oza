use clap::Parser;

/// OZA is the best
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}
fn main() {
    println!(
        "
            ███████    ███████████   █████████
          ███░░░░░███ ░█░░░░░░███   ███░░░░░███
         ███     ░░███░     ███░   ░███    ░███
        ░███      ░███     ███     ░███████████
        ░███      ░███    ███      ░███░░░░░███
        ░░███     ███   ████     █ ░███    ░███
         ░░░███████░   ███████████ █████   █████
           ░░░░░░░    ░░░░░░░░░░░ ░░░░░   ░░░░░

        "
    );
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
