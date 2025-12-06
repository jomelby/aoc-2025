fn get_ranges_and_ingredients(input: &str) -> (Vec<(i128, i128)>, Vec<i128>) {
    let mut ranges: Vec<(i128, i128)> = Vec::new();
    let mut ingredients: Vec<i128> = Vec::new();
    for line in input.lines() {
        if line.contains("-") {
            let first_number: i128 = line.split("-").collect::<Vec<&str>>()[0]
                .parse::<i128>()
                .unwrap();
            let second_number: i128 = line.split("-").collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();
            ranges.push((first_number, second_number));
        } else if line != "" {
            ingredients.push(line.parse::<i128>().unwrap())
        }
    }
    return (ranges, ingredients);
}

pub fn part1(input: &str) -> String {
    let mut spoiled_count = 0;
    let (ranges, ingredients) = get_ranges_and_ingredients(input);
    for ingredient in ingredients {
        for (start, end) in &ranges {
            if ingredient >= *start && ingredient <= *end {
                spoiled_count += 1;
                break;
            }
        }
    }
    return spoiled_count.to_string();
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;

    return sum.to_string();
}
