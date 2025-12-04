fn accessible(coordinates: (usize, usize), lines: &[&str], max_x: i128, max_y: i128) -> bool {
    let mut count = 0;
    let diffs_to_check: [(i8, i8); 8] = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for (x_diff, y_diff) in diffs_to_check {
        let (x, y) = (
            coordinates.0 as i128 - x_diff as i128,
            coordinates.1 as i128 - y_diff as i128,
        );
        if x >= 0 && x <= max_x && y >= 0 && y <= max_y {
            let line_chars: Vec<char> = lines[y as usize].chars().collect();
            if line_chars[x as usize] == '@' {
                count += 1;
            }
        }
        if count >= 4 {
            return false;
        }
    }
    return true;
}

pub fn part1(input: &str) -> String {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    let max_y = (lines.len() - 1) as i128;
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        let max_x = (line.len() - 1) as i128;
        for x in 0..line.len() {
            if line[x] == '@' {
                if accessible((x, y), &lines, max_x, max_y) {
                    sum += 1
                }
            }
        }
    }

    return sum.to_string();
}

fn accessible_rolls(input: &str) -> Vec<(usize, usize)> {
    let mut accessible_rolls: Vec<(usize, usize)> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let max_y = (lines.len() - 1) as i128;
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        let max_x = (line.len() - 1) as i128;
        for x in 0..line.len() {
            if line[x] == '@' {
                if accessible((x, y), &lines, max_x, max_y) {
                    accessible_rolls.push((x, y));
                }
            }
        }
    }
    return accessible_rolls;
}

fn get_new_input(input: &str, rolls_to_remove: Vec<(usize, usize)>) -> String {
    let mut new_input: Vec<Vec<char>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        let mut new_line: Vec<char> = Vec::new();
        for x in 0..line.len() {
            if rolls_to_remove.contains(&(x, y)) {
                new_line.push('.');
            } else {
                new_line.push(line[x]);
            }
        }
        new_input.push(new_line);
    }
    return new_input
        .into_iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;
    let mut new_input = input.to_string();
    loop {
        let accessible_rolls = accessible_rolls(&new_input);
        sum += accessible_rolls.len();
        if accessible_rolls.len() == 0 {
            break;
        }
        new_input = get_new_input(&new_input, accessible_rolls);
    }

    return sum.to_string();
}
