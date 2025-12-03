pub fn part1(input: &str) -> String {
    let mut position = 50;
    let mut count = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next();
        let distance: i32 = chars.as_str().parse().unwrap();
        if direction == Some('R') {
            position += distance;
            position = position % 100;
        } else {
            position += distance * -1;
            if position < 0 {
                // this means we have gone past 0 and we need to adjust
                position = 100 - (position % 100) * -1;
            }
        };
        if position == 0 {
            count += 1;
        } else if position == 100 {
            count += 1;
            position = 0;
        }
    }
    return count.to_string();
}

fn move_right(position: &mut i32, distance: i32) -> i32 {
    // change the position and return the count of times passing 0 or landing on 0
    *position += distance;
    let count = *position / 100;
    *position = *position % 100;

    return count;
}

fn move_left(position: &mut i32, distance: i32) -> i32 {
    let mut count: i32 = 0;
    let original_position: i32 = *position;
    *position += distance * -1;
    println!("{position}");
    if -100 < *position && *position < 0 {
        count += 1;
        // this means we have gone past 0 and we need to adjust
        *position = 100 - (*position % 100) * -1;
        if original_position == 0 {
            count -= 1;
        }
    } else if *position <= -100 {
        count += *position * -1 / 100 + 1;
        *position = 100 - (*position % 100) * -1;
        if original_position == 0 {
            count -= 1;
        }
    } else if *position == 0 {
        count += 1;
    }
    if *position == 100 {
        *position = 0;
    };
    println!("{count}");
    return count;
}

pub fn part2(input: &str) -> String {
    let mut position = 50;
    let mut count = 0;
    for line in input.lines() {
        // println!("{line} position: {position} count: {count}");
        let mut chars = line.chars();
        let direction = chars.next();
        let distance: i32 = chars.as_str().parse().unwrap();
        if direction == Some('R') {
            count += move_right(&mut position, distance);
        } else {
            count += move_left(&mut position, distance);
        };
        println!("{position} {count} {line}");
    }
    return count.to_string();
}

#[test]
fn test_move_right() {
    let mut position = 50;
    assert_eq!(move_right(&mut position, 50), 1);
    assert_eq!(position, 0);
    assert_eq!(move_right(&mut position, 250), 2);
    assert_eq!(position, 50);
    assert_eq!(move_right(&mut position, 300), 3);
    assert_eq!(position, 50);
}

#[test]
fn test_move_left() {
    let mut position = 50;
    assert_eq!(move_left(&mut position, 50), 1);
    assert_eq!(position, 0);
    assert_eq!(move_left(&mut position, 250), 2);
    assert_eq!(position, 50);
    assert_eq!(move_left(&mut position, 300), 3);
    assert_eq!(position, 50);
    assert_eq!(move_left(&mut position, 350), 4);
    assert_eq!(position, 0);
    assert_eq!(move_left(&mut position, 50), 0);
    assert_eq!(position, 50);
    position = 34;
    assert_eq!(move_left(&mut position, 961), 10);
    assert_eq!(position, 73);
}
