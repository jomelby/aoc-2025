pub mod days;

pub fn run_day(day: u8, input: &str) -> (String, String) {
    match day {
        1 => (days::day01::part1(input), days::day01::part2(input)),
        2 => (days::day02::part1(input), days::day02::part2(input)),
        3 => (days::day03::part1(input), days::day03::part2(input)),
        4 => (days::day04::part1(input), days::day04::part2(input)),
        5 => (days::day05::part1(input), days::day05::part2(input)),
        6 => (days::day06::part1(input), days::day06::part2(input)),
        7 => (days::day07::part1(input), days::day07::part2(input)),
        _ => panic!("Unknown day"),
    }
}
