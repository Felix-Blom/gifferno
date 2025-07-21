use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // GIF name
    #[arg(short, long)]
    name: String,
    // Optional path
    #[arg(short, long, default_value = "#")]
    print_character: String,
}

fn main() {
    let args = Args::parse();

    println!("{} {}", args.name, args.print_character);
}
