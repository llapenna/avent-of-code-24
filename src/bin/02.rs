advent_of_code::solution!(2);

fn get_reports(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .enumerate()
        .map(|(_, item)| {
            item.split_whitespace()
                .map(|string| string.parse::<u16>().unwrap())
                .collect()
        })
        .collect()
}

fn are_ordered(report_diff: Vec<i32>) -> bool {
    let abs_sum = report_diff
        .iter()
        .enumerate()
        .fold(0, |acc, (_, val)| acc + val.abs());
    let sum = report_diff
        .iter()
        .enumerate()
        .fold(0, |acc, (_, val)| acc + val);

    abs_sum == sum.abs()
}

fn is_safe(report: &Vec<u16>) -> bool {
    let mut differences: Vec<i32> = Vec::with_capacity(report.len() - 1);
    let mut valid = true;

    for (i, _) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let a = i32::from(report[i - 1]);
        let b = i32::from(report[i]);
        let diff = a - b;

        if diff.abs() > 3 || diff == 0 {
            valid = false;
            break;
        } else {
            differences.push(diff);
        }
    }

    if valid {
        valid = are_ordered(differences);
    }

    valid
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = get_reports(input);

    let mut safe = 0;
    for (_, report) in reports.iter().enumerate() {
        if is_safe(report) {
            safe += 1;
        }
    }

    Some(safe)
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
