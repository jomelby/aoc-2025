use std::collections::HashSet;

fn distance(a: (i128, i128, i128), b: (i128, i128, i128)) -> f64 {
    let dx = (b.0 - a.0) as f64;
    let dy = (b.1 - a.1) as f64;
    let dz = (b.2 - a.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn part1(input: &str) -> String {
    let junction_boxes: Vec<(i128, i128, i128)> = input
        .lines()
        .map(|line| {
            // parse three comma-separated values into a tuple
            let mut parts = line.split(',').map(|p| p.trim().parse::<i128>().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            (x, y, z)
        })
        .collect::<Vec<(i128, i128, i128)>>();
    // making a matrix of distances
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..junction_boxes.len() - 1 {
        for j in i + 1..junction_boxes.len() {
            distances.push((i, j, distance(junction_boxes[i], junction_boxes[j])));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for (a, b, _) in distances.iter().take(1000) {
        if circuits
            .iter()
            .any(|circuit| circuit.contains(a) || circuit.contains(b))
        {
            // possible we are joining two circuits
            let mut new_circuit: HashSet<usize> = HashSet::new();
            let mut circuit_idx_to_remove: Vec<usize> = Vec::new();
            for (idx, circuit) in circuits.iter().enumerate() {
                if circuit.contains(a) || circuit.contains(b) {
                    new_circuit.extend(circuit.iter());
                    circuit_idx_to_remove.push(idx);
                }
            }
            new_circuit.insert(*a);
            new_circuit.insert(*b);
            circuit_idx_to_remove
                .iter()
                .enumerate()
                .for_each(|(idx, index_to_remove)| {
                    circuits.remove(index_to_remove - idx);
                });
            circuits.push(new_circuit.iter().map(|f| *f).collect::<Vec<usize>>())
        } else {
            circuits.push(vec![*a, *b])
        }
    }
    circuits.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
    return circuits
        .iter()
        .take(3)
        .fold(1, |acc, x| acc * x.len())
        .to_string();
}

pub fn part2(input: &str) -> String {
    let junction_boxes: Vec<(i128, i128, i128)> = input
        .lines()
        .map(|line| {
            // parse three comma-separated values into a tuple
            let mut parts = line.split(',').map(|p| p.trim().parse::<i128>().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            (x, y, z)
        })
        .collect::<Vec<(i128, i128, i128)>>();
    let num_junction_boxes = junction_boxes.len();
    // making a matrix of distances
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..junction_boxes.len() - 1 {
        for j in i + 1..junction_boxes.len() {
            distances.push((i, j, distance(junction_boxes[i], junction_boxes[j])));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut answer: i128 = 0;
    for (a, b, _) in distances.iter() {
        if circuits
            .iter()
            .any(|circuit| circuit.contains(a) || circuit.contains(b))
        {
            // possible we are joining two circuits
            let mut new_circuit: HashSet<usize> = HashSet::new();
            let mut circuit_idx_to_remove: Vec<usize> = Vec::new();
            for (idx, circuit) in circuits.iter().enumerate() {
                if circuit.contains(a) || circuit.contains(b) {
                    new_circuit.extend(circuit.iter());
                    circuit_idx_to_remove.push(idx);
                }
            }
            new_circuit.insert(*a);
            new_circuit.insert(*b);
            circuit_idx_to_remove
                .iter()
                .enumerate()
                .for_each(|(idx, index_to_remove)| {
                    circuits.remove(index_to_remove - idx);
                });
            circuits.push(new_circuit.iter().map(|f| *f).collect::<Vec<usize>>())
        } else {
            circuits.push(vec![*a, *b])
        }
        if circuits[0].len() == num_junction_boxes {
            let junction_box_a = junction_boxes[*a];
            let junction_box_b = junction_boxes[*b];
            answer = junction_box_a.0 * junction_box_b.0;
            break;
        }
    }
    return answer.to_string();
}
