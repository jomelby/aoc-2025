use std::cmp::max;

pub fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        for range in line.split(",") {
            if let Some((left, right)) = range.split_once('-') {
                let start: i128 = left.parse().unwrap();
                let end: i128 = right.parse().unwrap();
                for i in start..end + 1 {
                    let len = i.to_string().len();
                    if len % 2 != 0 {
                        continue;
                    } else {
                        let first_half: String = i.to_string().chars().take(len / 2).collect();
                        let second_half: String = i.to_string().chars().skip(len / 2).collect();
                        if first_half == second_half {
                            sum += i
                        }
                    }
                }
            }
        }
    }
    return sum.to_string();
}

fn check_number(number: i128) -> i128 {
    let mut max_repeats: i128 = 0;
    let num_as_string = number.to_string();
    for i in 1..num_as_string.len() {
        if num_as_string.len() % i != 0 {
            continue;
        }
        let mut substrings: Vec<String> = Vec::new();
        for j in 0..num_as_string.len() / i {
            substrings.push(num_as_string.chars().skip(i * j).take(i).collect());
        }
        // check that all the substrings are the same
        let first_element = &substrings[0];
        // println!("{:?}", substrings);
        if substrings.iter().all(|element| element == first_element) {
            max_repeats = max(max_repeats, (num_as_string.len() / i) as i128);
            return max_repeats;
        }
    }
    return max_repeats;
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        for range in line.split(",") {
            if let Some((left, right)) = range.split_once('-') {
                let start: i128 = left.parse().unwrap();
                let end: i128 = right.parse().unwrap();
                for i in start..end + 1 {
                    if check_number(i) > 0 {
                        sum += i;
                    };
                }
            }
        }
    }
    return sum.to_string();
}

#[test]
fn test_part1_part2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\n1698522-1698528,446443-446449,38593856-38593862,565653-565659,\n824824821-824824827,2121212118-2121212124";
    assert_eq!(part1(input), "1227775554");
    assert_eq!(part2(input), "4174379265");
}

#[test]
fn test_check_number() {
    assert_eq!(check_number(1111111), 7);
    assert_eq!(check_number(1212121212), 5);
}
