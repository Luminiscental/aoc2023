use crate::day::Day;

const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn extract_digit(string: &str, allow_words: bool) -> Option<u32> {
    allow_words
        .then(|| {
            DIGITS
                .iter()
                .find_map(|&(name, value)| string.starts_with(name).then_some(value))
        })
        .flatten()
        .or_else(|| string.chars().next()?.to_digit(10))
}

fn first_last_digits(string: &str, allow_words: bool) -> Option<u32> {
    let (mut first, mut last) = (None, None);
    (0..string.len())
        .map(|i| &string[i..])
        .filter_map(|s| extract_digit(s, allow_words))
        .for_each(|digit| {
            first = first.or(Some(digit));
            last = Some(digit);
        });
    Some(first? * 10 + last?)
}

pub struct Day01;

impl<'a> Day<'a> for Day01 {
    const DAY: usize = 1;

    type Input = Vec<&'a str>;
    type ProcessedInput = Vec<&'a str>;

    fn parse(input: &'a str) -> Self::Input {
        input.trim().lines().collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .filter_map(|s| first_last_digits(s, false))
            .sum::<u32>();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .iter()
            .filter_map(|s| first_last_digits(s, true))
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day01 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const EXAMPLE2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_day01_examples() {
        let input = Day01::parse(EXAMPLE1);
        let (_, part1) = Day01::solve_part1(input);
        assert_eq!(part1, "142");

        let input = Day01::parse(EXAMPLE2);
        let (input, _) = Day01::solve_part1(input);
        let part2 = Day01::solve_part2(input);
        assert_eq!(part2, "281");
    }
}
