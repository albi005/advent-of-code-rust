use std::collections::HashMap;

advent_of_code::solution!(8);

struct Node {
    left: String,
    right: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut moves = lines.next().unwrap().chars().cycle();
    lines.next();
    let map = lines
        .map(|line| {
            let node = line[..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (node, Node { left, right })
        })
        .collect::<HashMap<_, _>>();

    let mut current = "AAA";
    let mut count = 0;
    while current != "ZZZ" {
        let node = map.get(current).unwrap();
        let next = if moves.next().unwrap() == 'L' {
            &node.left
        } else {
            &node.right
        };
        current = next;
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut moves = lines.next().unwrap().chars().cycle();
    lines.next();
    let map = lines
        .map(|line| {
            let node = line[..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (node, Node { left, right })
        })
        .collect::<HashMap<_, _>>();

    let mut positions: Vec<_> = map.keys().filter(|node| node.ends_with('A')).collect();
    let mut count = 0;
    while positions.iter().filter(|node| node.ends_with('Z')).count() < positions.len() {
        let left = moves.next().unwrap() == 'L';
        for i in 0..positions.len() {
            let node = map.get(positions[i]).unwrap();
            let next = if left { &node.left } else { &node.right };
            positions[i] = next;
        }
        count += 1;
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
