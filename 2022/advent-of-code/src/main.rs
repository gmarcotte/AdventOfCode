use clap::Parser;

mod input_utils;

mod d1;

#[derive(Parser,Default,Debug)]
#[clap(author="gmarcotte", version, about="Solutions to Advent of Code 2022")]
struct Arguments {
    /// Day of AoC to solve (1 - 25)
    day: usize,

    /// Input file path
    input: String,
}

fn main() {
    let args = Arguments::parse();
    println!("Solving day {} for input file {}", args.day, args.input);
    if let Ok(lines) = input_utils::read_lines(args.input) {
        match args.day {
            1 => d1::main(lines, 3),
            _ => println!("Day {} is not implemented", args.day),
        }
    } else {
        println!("Failed to read lines!");
    }
}
