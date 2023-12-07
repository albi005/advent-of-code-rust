advent_of_code::solution!(3);

struct Number {
    pub value: usize,
    pub first: usize,
    pub last: usize,
    pub valid: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut nums: Vec<Vec<Number>> = input
        .lines()
        .map(|line| {
            let mut rest_of_line = line;
            let mut offset = 0;
            let mut row: Vec<Number> = vec![];
            while let Some(mut first) = rest_of_line.find(|c: char| c.is_numeric()) {
                let mut last;
                if let Some(index) = rest_of_line[first..].find(|c: char| !c.is_numeric()) {
                    last = first + index - 1;
                } else {
                    last = rest_of_line.len() - 1;
                }
                let value = rest_of_line[first..(last + 1)].parse().unwrap();

                rest_of_line = &rest_of_line[(last + 1)..];

                first += offset;
                last += offset;
                row.push(Number { value, first, last, valid: false });

                offset = last + 1;
            };

            return row;
        })
        .collect();

    for (y, line) in input.lines().enumerate() {
        for x in line
            .chars()
            .enumerate()
            .filter(|(_, c)| !c.is_numeric() && c != &'.')
            .map(|(x, _)| x) {
            for dy in -1..=1 {
                let y = y as i32 + dy;
                for num in &mut (nums[y as usize]) {
                    if num.first.abs_diff(x) <= 1 || num.last.abs_diff(x) <= 1 {
                        num.valid = true;
                    }
                }
            };
        }
    };

    let sum: usize = nums.iter()
        .flatten()
        .filter(|num| num.valid)
        .map(|num| num.value)
        .sum();

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nums: Vec<Vec<Number>> = input
        .lines()
        .map(|line| {
            let mut rest_of_line = line;
            let mut offset = 0;
            let mut row: Vec<Number> = vec![];
            while let Some(mut first) = rest_of_line.find(|c: char| c.is_numeric()) {
                let mut last;
                if let Some(index) = rest_of_line[first..].find(|c: char| !c.is_numeric()) {
                    last = first + index - 1;
                } else {
                    last = rest_of_line.len() - 1;
                }
                let value = rest_of_line[first..(last + 1)].parse().unwrap();

                rest_of_line = &rest_of_line[(last + 1)..];

                first += offset;
                last += offset;
                row.push(Number { value, first, last, valid: false });

                offset = last + 1;
            };

            return row;
        })
        .collect();

    let mut sum = 0;

    for (y, line) in input.lines().enumerate() {
        for x in line
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'*')
            .map(|(x, _)| x) {
            let mut count = 0;
            let mut product = 1;
            for dy in -1..=1 {
                let y = y as i32 + dy;
                for num in &mut (nums[y as usize]) {
                    if num.first.abs_diff(x) <= 1 || num.last.abs_diff(x) <= 1 {
                        product *= num.value;
                        count += 1;
                    }
                }
            };
            if count == 2 {
                sum += product;
            }
        }
    };

    Some(sum as u32)
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
