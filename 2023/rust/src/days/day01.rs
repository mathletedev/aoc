use crate::solution::{Part, Solution};

const PART1: Part = |input| {
    input
        .lines()
        .map(|line| {
            // forward scan
            10 * line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
                // reverse scan
                + line
                    .chars()
                    .rfind(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
        })
        .sum::<u32>()
        .to_string()
};

const PART2: Part = |input| {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|&d| d.to_string())
    .collect::<Vec<String>>();
    let rev_digits = digits
        .iter()
        .map(|d| d.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    fn find_digit(s: String, digits: &[String]) -> u32 {
        // find first numerical character
        let numerical = s
            .chars()
            .enumerate()
            .find(|(_, c)| c.is_ascii_digit())
            .unwrap();
        // find first spelled digit
        let spelled = digits
            .iter()
            .enumerate()
            .map(|(i, d)| (s.find(d), i + 1))
            .filter(|(i, _)| i.is_some())
            .min_by(|(i, _), (j, _)| i.cmp(j));

        if let Some((i, d)) = spelled {
            // return earlier digit
            if i.unwrap() < numerical.0 {
                return d.try_into().unwrap();
            }
        }

        numerical.1.to_digit(10).unwrap()
    }

    input
        .lines()
        .map(|line| {
            10 * find_digit(line.to_string(), &digits)
                + find_digit(line.chars().rev().collect::<String>(), &rev_digits)
        })
        .sum::<u32>()
        .to_string()
};

pub const SOLUTION: Solution = Solution {
    part1: Some(PART1),
    part2: Some(PART2),
};
