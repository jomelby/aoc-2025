fn get_problems_and_operation(input: &str) -> (Vec<Vec<i128>>, Vec<&str>) {
    let mut problems: Vec<Vec<i128>> = Vec::new();
    let mut operations: Vec<&str> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    // get the number of problems to initialize the problem vector
    lines[0]
        .split_whitespace()
        .enumerate()
        .for_each(|(_idx, _)| problems.push(Vec::new()));
    for i in 0..lines.len() {
        // last line has the operations
        if i == lines.len() - 1 {
            lines[i]
                .split_whitespace()
                .for_each(|operator| operations.push(operator));
        } else {
            lines[i]
                .split_whitespace()
                .enumerate()
                .for_each(|(idx, value)| problems[idx].push(value.parse::<i128>().unwrap()));
        }
    }
    return (problems, operations);
}

fn get_problems_and_operation_part2(input: &str) -> (Vec<Vec<i128>>, Vec<String>) {
    // transpose the string from something like "aaa\nbbb" to "ab\nab\nab"
    let mut transposed_input: Vec<Vec<char>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    lines[0]
        .chars()
        .for_each(|_| transposed_input.push(Vec::new()));
    // reverse the line along the way
    // last character of the line is the first element of the new line
    lines.iter().for_each(|line| {
        line.chars()
            .rev()
            .enumerate()
            .for_each(|(char_idx, char)| transposed_input[char_idx].push(char))
    });
    let transposed_lines: String = transposed_input
        .into_iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    let mut problems: Vec<Vec<i128>> = Vec::new();
    let mut temp_problems: Vec<i128> = Vec::new();
    let mut operations: Vec<String> = Vec::new();
    for line in transposed_lines.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let line_chars: Vec<char> = line.chars().collect();
        let number: i128 = line[..line.len() - 1].trim().parse().unwrap();
        let last_char = line_chars.last().unwrap();
        temp_problems.push(number);
        if *last_char == '*' || *last_char == '+' {
            problems.push(temp_problems);
            temp_problems = Vec::new();
            operations.push(last_char.to_string());
        }
    }
    return (problems, operations);
}

fn evaluate_problem(values: Vec<i128>, operator: &str) -> i128 {
    let answer: i128 = match operator {
        "*" => values.iter().fold(1, |acc, x| x * acc),
        "+" => values.iter().sum(),
        _ => panic!("Unknown operator"),
    };
    return answer;
}

pub fn part1(input: &str) -> String {
    let (values, operators) = get_problems_and_operation(input);
    let sum: i128 = values
        .iter()
        .enumerate()
        .map(|(idx, numbers)| evaluate_problem(numbers.clone(), operators[idx]))
        .sum();
    return sum.to_string();
}

pub fn part2(input: &str) -> String {
    let (values, operators) = get_problems_and_operation_part2(input);
    let sum: i128 = values
        .iter()
        .enumerate()
        .map(|(idx, numbers)| evaluate_problem(numbers.clone(), &operators[idx]))
        .sum();
    return sum.to_string();
}
