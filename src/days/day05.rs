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

fn count_new_fresh_ingredients(
    ranges_checked: &Vec<(i128, i128)>,
    range_to_check: &(i128, i128),
) -> i128 {
    let max_ingredient_ids = range_to_check.1 - range_to_check.0 + 1;
    let overlap: i128 = ranges_checked
        .iter()
        .fold(max_ingredient_ids, |acc, range| {
            acc - ingredient_overlap(*range, *range_to_check)
        });
    return overlap;
}

fn count_new_fresh_ingredients_v2(
    ranges_checked: &Vec<(i128, i128)>,
    range_to_check: &(i128, i128),
) -> i128 {
    let mut new_ingredient_ranges: Vec<(i128, i128)> = Vec::from([*range_to_check]);
    for range_checked in ranges_checked {
        let mut new_new_ingredient_ranges: Vec<(i128, i128)> = Vec::new();
        for new_ingredient_range in new_ingredient_ranges {
            range_overlap(*range_checked, new_ingredient_range)
                .iter()
                .for_each(|overlapping_range| new_new_ingredient_ranges.push(*overlapping_range));
        }
        new_ingredient_ranges = new_new_ingredient_ranges;
    }
    return new_ingredient_ranges
        .iter()
        .map(|(start, finish)| finish - start + 1)
        .sum();
}

fn ingredient_overlap(reference_range: (i128, i128), new_range: (i128, i128)) -> i128 {
    // if the start is in the middle of a previously counted range

    if new_range.0 >= reference_range.0 && new_range.0 <= reference_range.1 {
        if new_range.1 >= reference_range.1 {
            // for example if you had 1,5 and 4,6 you would have an overlap of 2
            return reference_range.1 - new_range.0 + 1;
        } else {
            // whole range overlaps
            // for example if you had 1,9 and 4, 5 you would have an overlap of 2
            return new_range.1 - new_range.0 + 1;
        }
    }
    // 4,6 and 5,9
    if new_range.1 >= reference_range.0 && new_range.1 <= reference_range.1 {
        return new_range.1 - reference_range.0 + 1;
    }
    // if reference range is inside the new range return the length of the reference range
    // reference 13,13 new range 12,14
    if reference_range.0 >= new_range.0 && reference_range.1 <= new_range.1 {
        return reference_range.1 - reference_range.0 + 1;
    }
    return 0;
}

fn range_overlap(reference_range: (i128, i128), new_range: (i128, i128)) -> Vec<(i128, i128)> {
    let mut unique_ranges: Vec<(i128, i128)> = Vec::new();
    // if the start is in the middle of a previously counted range
    if new_range.0 >= reference_range.0 && new_range.0 <= reference_range.1 {
        if new_range.1 > reference_range.1 {
            // for example if you had 1,5 and 4,6 you would have a unique range of 5,6
            unique_ranges.push((reference_range.1 + 1, new_range.1));
            return unique_ranges;
        } else {
            // whole range overlaps
            // for example if you had 1,9 and 4, 5 you would have an overlap of 2
            return unique_ranges;
        }
    }
    // 5,9 and 4,6 should return 4,4
    if new_range.1 >= reference_range.0 && new_range.1 <= reference_range.1 {
        unique_ranges.push((new_range.0, reference_range.0 - 1));
        return unique_ranges;
    }
    // if reference range is inside the new range return the length of the reference range
    // reference 13,13 new range 12,14
    if reference_range.0 >= new_range.0 && reference_range.1 <= new_range.1 {
        unique_ranges.push((new_range.0, reference_range.0 - 1));
        unique_ranges.push((reference_range.1 + 1, new_range.1));

        return unique_ranges;
    }
    return Vec::from([new_range]);
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;
    let (ranges, _) = get_ranges_and_ingredients(input);
    let mut ranges_checked: Vec<(i128, i128)> = Vec::new();
    for range in ranges.iter() {
        sum += std::cmp::max(count_new_fresh_ingredients_v2(&ranges_checked, &range), 0);
        ranges_checked.push(*range);
    }
    return sum.to_string();
}

#[test]
fn test_ingredient_overlap() {
    assert_eq!(ingredient_overlap((13, 13), (12, 18)), 1);
    assert_eq!(ingredient_overlap((12, 18), (13, 13)), 1);
}

#[test]
fn test_new_ingredients() {
    assert_eq!(
        count_new_fresh_ingredients(&Vec::from([(13, 13), (10, 14)]), &(12, 18)),
        4
    );
}

#[test]
fn test_new_ingredients_v2() {
    assert_eq!(
        count_new_fresh_ingredients_v2(&Vec::from([(13, 13), (10, 14)]), &(12, 18)),
        4
    );
}

#[test]
fn test_range_overlap() {
    assert_eq!(range_overlap((1, 5), (4, 6)), Vec::from([(6, 6)]));
    assert_eq!(range_overlap((2, 6), (6, 7)), Vec::from([(7, 7)]));
    assert_eq!(range_overlap((2, 6), (4, 8)), Vec::from([(7, 8)]));
    assert_eq!(range_overlap((2, 6), (0, 8)), Vec::from([(0, 1), (7, 8)]));
    assert_eq!(range_overlap((16, 20), (12, 18)), Vec::from([(12, 15)]));
}
