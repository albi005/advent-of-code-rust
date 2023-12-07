use std::collections::HashSet;

advent_of_code::solution!(4);

fn get_wins(line: &str) -> usize {
    let colon = line.find(':').unwrap();
    let bar = line.find('|').unwrap();
    let winners: HashSet<u32> = line[colon + 2..bar - 1]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let guesses: HashSet<u32> = line[bar + 2..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    winners.intersection(&guesses).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let wins = get_wins(line);

                if wins == 0 {
                    0
                } else {
                    2u32.pow(wins as u32 - 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let wins: Vec<usize> = input.lines().map(get_wins).collect();

    let mut points: Vec<usize> = vec![1; wins.len()];
    for i in 0..points.len() {
        let wins = wins[i];
        for j in (i + 1)..((i + 1 + wins).min(points.len())) {
            points[j] += points[i];
        }
    }

    return Some(points.iter().sum::<usize>() as u32);
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
