use std;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted: Vec<&str> = input.split_whitespace().collect();

    let mut column_a: Vec<u32> = Vec::with_capacity(splitted.len());
    let mut column_b: Vec<u32> = Vec::with_capacity(splitted.len());

    // The split on the input outputs a single array of strings, so we're
    // matching the index to determine if it's an A or B column.
    for (i, item) in splitted.iter().enumerate() {
        let number = item.parse::<u32>().unwrap();
        let is_a = i % 2 == 0;

        if is_a {
            column_a.push(number);
        } else {
            column_b.push(number);
        }
    }

    column_a.sort_by(|a, b| a.cmp(b));
    column_b.sort_by(|a, b| a.cmp(b));

    let mut sum: u32 = 0;
    for (i, _) in column_a.iter().enumerate() {
        let a = column_a[i];
        let b = column_b[i];

        sum += a.abs_diff(b);
    }

    Some(sum)
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
