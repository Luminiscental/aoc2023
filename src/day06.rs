use crate::day::Day;

const EPSILON: f64 = 0.00001;

fn harsh_floor(t: f64) -> i64 {
    let f = t.floor();
    let d = if (t - f).abs() < EPSILON { 1 } else { 0 };
    f as i64 - d
}

fn count_ways(time: i64, distance: i64) -> i64 {
    let disc = time * time - 4 * distance;
    if disc < 0 {
        time + 1
    } else {
        let r = (disc as f64).sqrt();
        let (r1, r2) = ((time as f64 - r) / 2.0, (time as f64 + r) / 2.0);
        harsh_floor(r2).min(time) + harsh_floor(-r1).min(0) + 1
    }
}

pub struct Day06;

impl<'a> Day<'a> for Day06 {
    const DAY: usize = 6;

    type Input = (&'a str, &'a str);
    type ProcessedInput = (&'a str, &'a str);

    fn parse(input: &'a str) -> Self::Input {
        let mut lines = input.trim().lines();
        let mut eat = || lines.next().unwrap();
        (eat(), eat())
    }

    fn solve_part1((times, distances): Self::Input) -> (Self::ProcessedInput, String) {
        let get_nums = |s: &'a str| {
            s.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse().unwrap())
        };
        let ans = get_nums(times)
            .zip(get_nums(distances))
            .map(|(t, d)| count_ways(t, d))
            .product::<i64>();
        ((times, distances), ans.to_string())
    }

    fn solve_part2((times, distances): Self::ProcessedInput) -> String {
        let get_num = |s: &'a str| {
            s.chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap()
        };
        count_ways(get_num(times), get_num(distances)).to_string()
    }
}

#[cfg(test)]
mod test_day06 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_day06_examples() {
        let input = Day06::parse(EXAMPLE);
        let (input, part1) = Day06::solve_part1(input);
        let part2 = Day06::solve_part2(input);
        assert_eq!(part1, "288");
        assert_eq!(part2, "71503");
    }
}
