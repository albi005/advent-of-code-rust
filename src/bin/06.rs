advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap());
    let bests = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap());

    // speed = x
    // move_time = time - x
    // dist = speed * move_time
    // dist = x * (time - x)
    // dist = -x^2 + x*time
    //
    // -x^2 + time*x > best
    // -x^2 + time*x - best > 0
    //
    // x1 = (-time + sqrt(time^2 - 4 * -1 * -best)) / -2
    // x2 = (-time - sqrt(time^2 - 4 * -1 * -best)) / -2

    let res = times
        .zip(bests)
        .map(|(time, best)| {
            let time = time as f64;
            let best = best as f64;
            let mut x1 = (-time + f64::sqrt(time*time - 4.0 * -1.0 * -best)) / -2.0;
            let mut x2 = (-time - f64::sqrt(time*time - 4.0 * -1.0 * -best)) / -2.0;
            x1 += 0.1; // for good luck
            x2 -= 0.1;
            let x1 = x1.ceil() as i64;
            let x2 = x2.floor() as i64;
            let res = (x2 - x1 + 1) as u32;
            println!("{}", res);
            res
        })
        .product();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .skip(10)
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let best = lines
        .next()
        .unwrap()
        .chars()
        .skip(10)
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    // speed = x
    // move_time = time - x
    // dist = speed * move_time
    // dist = x * (time - x)
    // dist = -x^2 + x*time
    //
    // -x^2 + time*x > best
    // -x^2 + time*x - best > 0
    //
    // x1 = (-time + sqrt(time^2 - 4 * -1 * -best)) / -2
    // x2 = (-time - sqrt(time^2 - 4 * -1 * -best)) / -2

    let time = time as f64;
    let best = best as f64;
    let mut x1 = (-time + f64::sqrt(time*time - 4.0 * -1.0 * -best)) / -2.0;
    let mut x2 = (-time - f64::sqrt(time*time - 4.0 * -1.0 * -best)) / -2.0;
    x1 += 0.1; // for good luck
    x2 -= 0.1;
    let x1 = x1.ceil() as i64;
    let x2 = x2.floor() as i64;
    let res = (x2 - x1 + 1) as u32;
    println!("{}", res);

    Some(res)
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
