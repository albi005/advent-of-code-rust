advent_of_code::solution!(5);

struct MapOption {
    first: i64,
    last: i64,
    delta: i64,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let seeds: Vec<i64> = lines[0][7..]
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let maps: Vec<Vec<MapOption>> = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|line| {
                    let mut parts = line.split(' ').map(|s| s.parse::<i64>().unwrap());
                    let dest_start = parts.next().unwrap();
                    let src_start = parts.next().unwrap();
                    let len = parts.next().unwrap();

                    let first = src_start;
                    let last = src_start + len - 1;
                    let delta = dest_start - src_start;
                    MapOption { first, last, delta }
                })
                .collect()
        })
        .collect();

    let result = seeds
        .iter()
        .map(|seed| {
            let mut val = *seed;
            for map in &maps {
                if let Some(option) = map
                    .iter()
                    .filter(|opt| &val >= &opt.first && &val <= &opt.last)
                    .next()
                {
                    val += option.delta;
                };
            }
            val
        })
        .min()
        .unwrap();
    Some(result.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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