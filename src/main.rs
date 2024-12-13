mod day1;
mod day2;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Day1 {
        #[arg(short, long)]
        part: i32,
        #[arg(short, long)]
        input: std::path::PathBuf,
	},
    Day2 {
        #[arg(short, long)]
        part: i32,
        #[arg(short, long)]
        input: std::path::PathBuf,
	},

}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Day1{part, input} => day1::main(part, input),
        Commands::Day2{part, input} => day2::main(part, input),
    }
}
