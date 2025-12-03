pub mod days;

pub fn run_day(day: u8, input: &str) -> (String, String) {
    match day {
        1 => (days::day01::part1(input), days::day01::part2(input)),
        2 => (days::day02::part1(input), days::day02::part2(input)),
        _ => panic!("Unknown day"),
    }
}
