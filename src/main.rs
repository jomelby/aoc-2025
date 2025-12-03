use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let day = args.day.unwrap_or(1);

    let input = std::fs::read_to_string(format!("inputs/day{:02}.txt", day)).unwrap();
    let (p1, p2) = aoc_2025::run_day(day, &input);

    println!("Day {day} Part 1: {p1}");
    println!("Day {day} Part 2: {p2}");
}
