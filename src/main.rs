use anyhow::Result;
use clap::{Parser, Subcommand};
use patharg::InputArg;

mod day01;
mod day05;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "aoc")]
#[command(about = "Rusty Advent of Code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t)]
    input: InputArg,

    /// Run the second part of the daily challenge
    #[arg(short, long)]
    two: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Day01,
    Day05
}

fn line_vec(input: InputArg) -> Result<Vec<String>> {
    let mut the_vec: Vec<String> = Vec::new();
    for r in input.lines()? {
        the_vec.push(r?);
    }
    Ok(the_vec)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let r: Result<String> = match args.command {
        Commands::Day01 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day01::part1(lines)
            } else {
                day01::part2(lines)
            }
        },
        Commands::Day05 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day05::part1(lines)
            } else {
                day05::part2(lines)
            }
        },
    };
    println!("Result: {}", r?);
    Ok(())
}
