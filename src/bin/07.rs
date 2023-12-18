use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    High,
}

fn add_j(type_: Type, j: usize) -> Type {
    if j == 0 {
        return type_;
    }
    add_j(
        match type_ {
            Type::Five => Type::Five,
            Type::Four => Type::Five,
            Type::Full => unreachable!(),
            Type::Three => Type::Four,
            Type::TwoPair => Type::Full,
            Type::OnePair => Type::Three,
            Type::High => Type::OnePair,
        },
        j - 1,
    )
}

fn get_type_two(input: &str) -> Type {
    let counter = input
        .chars()
        .filter(|c| c != &'J')
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
    let mut vec: Vec<_> = counter.values().collect();
    vec.sort();

    let j = input.chars().filter(|c| c == &'J').count();

    add_j(
        match vec.as_slice() {
            [5] => Type::Five,
            [.., 4] => Type::Four,
            [2, 3] => Type::Full,
            [.., 3] => Type::Three,
            [.., 2, 2] => Type::TwoPair,
            [.., 2] => Type::OnePair,
            _ => Type::High,
        },
        j,
    )
}

struct Hand {
    cards: Vec<u8>,
    value: u32,
    type_: Type,
}

fn get_value_one(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec = input
        .lines()
        .map(|line| {
            let cards = &line[..5];
            Hand {
                cards: cards.chars().map(get_value_one).collect(),
                value: line[6..].parse::<u32>().unwrap(),
                type_: get_type_one(cards),
            }
        })
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| {
        if a.type_ == b.type_ {
            for i in 0..5 {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
            return Ordering::Equal;
        } else {
            return b.type_.cmp(&a.type_);
        }
    });

    let mut sum = 0;
    for (i, hand) in vec.iter().enumerate() {
        sum += hand.value * (i as u32 + 1);
    }
    Some(sum)
}

fn get_value_two(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn get_type_one(input: &str) -> Type {
    let counter = input.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let mut vec: Vec<_> = counter.values().collect();
    vec.sort();

    match vec.as_slice() {
        [5] => Type::Five,
        [1, 4] => Type::Four,
        [2, 3] => Type::Full,
        [1, 1, 3] => Type::Three,
        [1, 2, 2] => Type::TwoPair,
        [1, 1, 1, 2] => Type::OnePair,
        _ => Type::High,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec = input
        .lines()
        .map(|line| {
            let cards = &line[..5];
            Hand {
                cards: cards.chars().map(get_value_two).collect(),
                value: line[6..].parse::<u32>().unwrap(),
                type_: get_type_two(cards),
            }
        })
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| {
        if a.type_ == b.type_ {
            for i in 0..5 {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
            return Ordering::Equal;
        } else {
            return b.type_.cmp(&a.type_);
        }
    });

    let mut sum = 0;
    for (i, hand) in vec.iter().enumerate() {
        sum += hand.value * (i as u32 + 1);
    }
    Some(sum)
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
