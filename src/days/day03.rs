use std::i128;

fn parse_line(line: &str) -> i8 {
    // line looks like a long list of integers, find the largest number you can find
    // can take any two integers, but they must be in the same order
    let mut first_number = 0;
    let mut second_number = 0;
    let values: Vec<i8> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    for i in 0..values.len() {
        let value = values[i];
        if value > first_number && i != values.len() - 1 {
            first_number = value;
            second_number = 0;
        } else if value > second_number {
            second_number = value;
        }
    }
    let answer: i8 = (first_number.to_string() + &second_number.to_string())
        .parse()
        .unwrap();
    return answer;
}

fn find_largest_digit(values: &[i8]) -> (usize, i8) {
    // find the largest digit and return its index and value
    let mut largest_value: i8 = 0;
    let mut index: usize = 0;
    for i in 0..values.len() {
        let value = values[i];
        if value > largest_value {
            largest_value = value;
            index = i;
        }
    }
    return (index, largest_value);
}

fn parse_line_part2(line: &str) -> i128 {
    // line looks like a long list of integers, find the largest number you can find
    // can take any two integers, but they must be in the same order
    let length_to_find = 12;
    let mut found_values: Vec<i8> = Vec::new();
    let values: Vec<i8> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    let mut next_index = 0;
    for i in 0..length_to_find {
        let end = values.len() - length_to_find + i + 1;
        // this means we just need to take the rest
        if end <= 0 {
            values[next_index..]
                .iter()
                .for_each(|f| found_values.push(*f));
            break;
        }
        let (found_idx, val) = find_largest_digit(&values[next_index..end]);
        found_values.push(val);
        next_index += found_idx + 1;
    }
    let answer: String = found_values.iter().map(|f| f.to_string()).collect();
    return answer.parse::<i128>().unwrap();
}

pub fn part1(input: &str) -> String {
    let mut answer: i128 = 0;
    for line in input.lines() {
        answer += parse_line(line) as i128;
    }
    return answer.to_string();
}

pub fn part2(input: &str) -> String {
    let mut answer: i128 = 0;
    for line in input.lines() {
        answer += parse_line_part2(line);
    }
    return answer.to_string();
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("987654321111111"), 98);
    assert_eq!(parse_line("818181911112111"), 92)
}

#[test]
fn test_parse_line_part2() {
    assert_eq!(parse_line_part2("987654321111111"), 987654321111);
    assert_eq!(parse_line_part2("818181911112111"), 888911112111);
    assert_eq!(parse_line_part2("234234234234278"), 434234234278);
    assert_eq!(parse_line_part2("811111111111119"), 811111111119);
}
