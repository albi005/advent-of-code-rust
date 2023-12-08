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

fn get_min(
    first: i64,
    last: i64,
    maps: &Vec<Vec<MapOption>>,
    layer: usize,
    checked_options: usize,
) -> i64 {
    if layer == maps.len() {
        return first;
    };

    let map = &maps[layer];

    let first = first;
    let last = last;

    for (i, opt) in map.iter().enumerate().skip(checked_options) {
        //   ----
        //       --
        if opt.first > last {
            continue;
        }

        //   ----
        // --
        if opt.last < first {
            continue;
        }

        //   ----
        //    --
        if opt.first > first && opt.last < last {
            return get_min(first, opt.first - 1, maps, layer, i + 1)
                .min(get_min(
                    opt.first + opt.delta,
                    opt.last + opt.delta,
                    maps,
                    layer + 1,
                    0,
                ))
                .min(get_min(opt.last + 1, last, maps, layer, i + 1));
        }

        //   ----
        //  --
        if opt.last < last {
            return get_min(first + opt.delta, opt.last + opt.delta, maps, layer + 1, 0)
                .min(get_min(opt.last + 1, last, maps, layer, i + 1));
        }

        //   ----
        //      --
        if opt.first > first {
            return get_min(first, opt.first - 1, maps, layer, i + 1).min(get_min(
                opt.first + opt.delta,
                last + opt.delta,
                maps,
                layer + 1,
                0,
            ));
        }

        //   ----
        //  ------
        return get_min(first + opt.delta, last + opt.delta, maps, layer + 1, 0);
    }

    get_min(first, last, maps, layer + 1, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut seeds = lines[0][7..].split(' ').map(|s| s.parse::<i64>().unwrap());

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

    let mut min = maps.last().unwrap().first().unwrap().first;

    while let Some(first) = seeds.next() {
        let len = seeds.next().unwrap();
        min = get_min(first, first + len - 1, &maps, 0, 0).min(min);
    }

    Some(min as u32)
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
