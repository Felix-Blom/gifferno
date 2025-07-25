use core::str;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

const DEFAULT_CHARACTER: &str = "#";

#[derive(Subcommand, Debug)]
enum Commands {
    // Create and run a gif in the terminal without saving
    Run {
        #[arg(short, long)]
        file_path: String,
        #[arg(short, long, default_value = DEFAULT_CHARACTER)]
        print_character: String,
    },
    // Save the GIF by name to the database, without running
    Save {
        #[arg(short, long)]
        file_path: String,
        #[arg(short, long)]
        name: String,
    },
    // Get the GIF by name and print it in the terimnal using the `print_character`
    Get {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = DEFAULT_CHARACTER)]
        print_character: String,
    },
    // Remove a gif by saved name
    Delete {
        #[arg(short, long)]
        name: String,
    },
    // Clear the entire database
    Clear {},
    // Get a list of saved GIFS and pick the one you want
    Pick {},
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run {
            file_path,
            print_character,
        } => {
            println!("Showing {} with {}", file_path, print_character);
        }
        Commands::Save { file_path, name } => {
            println!("Saving GIF from file_path {} to name: {} ", file_path, name);
        }
        Commands::Get {
            name,
            print_character,
        } => {
            println!("Getting: {} with character {}", name, print_character);
        }
        Commands::Pick {} => {
            println!("Pick your GIF from the list");
        }
        Commands::Delete { name } => {
            println!("Are you sure you want to delete {name}");
            println!("Deleting {name}")
        }
        Commands::Clear {} => {
            println!("Are you sure you want to clear the entire database Y/N?");
            println!("Removing entire database.")
        }
    }
}
