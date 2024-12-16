use anyhow::Result;
use clap::{Parser, Subcommand};
use patharg::InputArg;

mod common;
mod day01;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

use crate::common::Pos;

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
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
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
        }
        Commands::Day05 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day05::part1(lines)
            } else {
                day05::part2(lines)
            }
        }
        Commands::Day06 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day06::part1(lines)
            } else {
                day06::part2(lines)
            }
        }
        Commands::Day07 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day07::part1(lines)
            } else {
                day07::part2(lines)
            }
        }
        Commands::Day08 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day08::part1(lines)
            } else {
                day08::part2(lines)
            }
        }
        Commands::Day09 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day09::part1(lines)
            } else {
                day09::part2(lines)
            }
        }
        Commands::Day10 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day10::part1(lines)
            } else {
                day10::part2(lines)
            }
        }
        Commands::Day11 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day11::part1(lines, 25)
            } else {
                day11::part2(lines, 75)
            }
        }
        Commands::Day12 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day12::part1(lines)
            } else {
                day12::part2(lines)
            }
        }
        Commands::Day13 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day13::part1(lines)
            } else {
                day13::part2(lines)
            }
        }
        Commands::Day14 => {
            let lines = line_vec(args.input)?;
            if !args.two {
                day14::part1(lines, Pos::new(101, 103)?)
            } else {
                day14::part2(lines)
            }
        }
    };
    println!("Result: {}", r?);
    Ok(())
}
