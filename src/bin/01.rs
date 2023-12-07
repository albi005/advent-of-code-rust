advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut first: u32 = 10;
                let mut last: u32 = 10;
                for c in line.chars() {
                    if let Some(digit) = c.to_digit(10) {
                        if first == 10 {
                            first = digit
                        }
                        last = digit;
                    }
                }

                10 * first + last
            })
            .sum(),
    )
}

fn to_digit(s: &str) -> Option<u32> {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, digit) in DIGITS.iter().enumerate() {
        if s.starts_with(digit) {
            return Some((i + 1).try_into().unwrap());
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut first: u32 = 10;
                let mut last: u32 = 10;
                for (i, c) in line.chars().enumerate() {
                    if let Some(digit) = c.to_digit(10) {
                        if first == 10 {
                            first = digit
                        };
                        last = digit;
                    } else if let Some(digit) = to_digit(&line[i..]) {
                        if first == 10 {
                            first = digit
                        };
                        last = digit;
                    }
                }

                10 * first + last
            })
            .sum(),
    )
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
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
