advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let skip_index = line.find(':').unwrap() + 2;
                let skipped = &line[skip_index..];

                let restarts = skipped.split("; ");
                for restart in restarts {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    let colors = restart.split(", ");
                    for color in colors {
                        let mut parts = color.split(' ');
                        let count: u32 = parts.next().unwrap().parse().unwrap();
                        let color = parts.next().unwrap();
                        match color {
                            "red" => r += count,
                            "green" => g += count,
                            "blue" => b += count,
                            _ => {},
                        }
                    }
                    if r > 12 || g > 13 || b > 14 {
                        return 0;
                    }
                }

                return i as u32 + 1;
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let skip_index = line.find(':').unwrap() + 2;
                let skipped = &line[skip_index..];
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                let restarts = skipped.split("; ");
                for restart in restarts {
                    let colors = restart.split(", ");
                    for color in colors {
                        let mut parts = color.split(' ');
                        let count: u32 = parts.next().unwrap().parse().unwrap();
                        let color = parts.next().unwrap();
                        match color {
                            "red" => if r < count { r = count },
                            "green" => if g < count { g = count },
                            "blue" => if b < count { b = count },
                            _ => {},
                        }
                    }
                }

                return r * g * b
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
