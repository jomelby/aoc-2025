use std::collections::{HashMap, HashSet};

fn find_starting_location(input: &str) -> (usize, usize) {
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        if line.contains("S") {
            return (line.find("S").unwrap(), i);
        }
    }
    panic!("no staring point found")
}

pub fn part1(input: &str) -> String {
    let starting_location = find_starting_location(input);
    let mut locations_to_check: Vec<(usize, usize)> = Vec::new();
    let mut locations_checked: HashSet<(usize, usize)> = HashSet::new();
    let diagram: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_y = diagram.len() - 1;
    let max_x = diagram[0].len();
    locations_to_check.push((starting_location.0, starting_location.1 + 1));
    let mut number_of_splits = 0;
    while locations_to_check.len() > 0 {
        let location_to_check = locations_to_check.pop().unwrap();
        if location_to_check.0 > max_x
            || location_to_check.1 > max_y
            || locations_checked.contains(&location_to_check)
        {
            continue;
        }
        locations_checked.insert(location_to_check);
        let char_at_location = diagram[location_to_check.1][location_to_check.0];
        if char_at_location == '^' {
            locations_to_check.push((location_to_check.0 - 1, location_to_check.1));
            locations_to_check.push((location_to_check.0 + 1, location_to_check.1));
            number_of_splits += 1;
        } else {
            locations_to_check.push((location_to_check.0, location_to_check.1 + 1));
        }
    }

    return number_of_splits.to_string();
}

fn get_next_locations(
    location: (usize, usize, usize),
    diagram: &Vec<Vec<char>>,
) -> Vec<(usize, usize, usize)> {
    let char_at_location = diagram[location.1][location.0];
    let mut next_locations: Vec<(usize, usize, usize)> = Vec::new();
    if char_at_location == '^' {
        next_locations.push((location.0 + 1, location.1 + 1, location.2));
        next_locations.push((location.0 - 1, location.1 + 1, location.2));
    } else {
        next_locations.push((location.0, location.1 + 1, location.2))
    }
    return next_locations;
}

pub fn part2(input: &str) -> String {
    let starting_location = find_starting_location(input);
    // x, y, path count
    // going to keep track of the number of times that path has been reached
    let mut locations_to_check: Vec<(usize, usize, usize)> = Vec::new();
    let diagram: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_y = diagram.len() - 1;
    locations_to_check.push((starting_location.0, starting_location.1 + 1, 1));
    let mut number_of_completions: i128 = 0;
    while locations_to_check.len() > 0 {
        let next_locations: Vec<(usize, usize, usize)> = locations_to_check
            .iter()
            .flat_map(|location| get_next_locations(*location, &diagram))
            .collect();
        let mut next_location_map: HashMap<(usize, usize), usize> = HashMap::new();
        for (x, y, count) in next_locations {
            let existing_value = next_location_map.get(&(x, y));
            if existing_value == None {
                next_location_map.insert((x, y), count);
            } else {
                next_location_map.insert((x, y), existing_value.unwrap() + count);
            }
        }
        locations_to_check = Vec::new();
        for ((next_x, next_y), next_count) in next_location_map.iter() {
            if *next_y == max_y {
                number_of_completions += *next_count as i128;
            } else {
                locations_to_check.push((*next_x, *next_y, *next_count));
            }
        }
    }

    return number_of_completions.to_string();
}
