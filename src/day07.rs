use std::cmp::Ordering;

use crate::day::Day;

fn hand_values(hand: &str) -> [usize; 5] {
    let val = |c: u8| {
        "0123456789TJQKA"
            .as_bytes()
            .iter()
            .position(|&b| b == c)
            .unwrap()
    };
    let bs = hand.as_bytes();
    [val(bs[0]), val(bs[1]), val(bs[2]), val(bs[3]), val(bs[4])]
}

fn hand_strength(mut values: [usize; 5], with_jokers: bool) -> usize {
    if with_jokers {
        if let Some(i) = values.iter().position(|&v| v == 0) {
            return (1..15)
                .map(|v| {
                    values[i] = v;
                    hand_strength(values, with_jokers)
                })
                .max()
                .unwrap();
        }
    }
    let mut counts = [0; 15];
    (0..5).for_each(|i| counts[values[i]] += 1);
    counts.sort_unstable();
    match counts[14] {
        n if n > 3 => n + 1,
        3 if counts[13] == 2 => 4,
        3 => 3,
        2 if counts[13] == 2 => 2,
        2 => 1,
        _ => 0,
    }
}

fn score(bids: &mut [([usize; 5], usize)], with_jokers: bool) -> usize {
    bids.sort_by(|a, b| {
        hand_strength(a.0, with_jokers)
            .cmp(&hand_strength(b.0, with_jokers))
            .then(a.0.cmp(&b.0))
    });
    bids.iter()
        .enumerate()
        .map(|(i, (_h, b))| (i + 1) * b)
        .sum::<usize>()
}

pub struct Day07;

impl<'a> Day<'a> for Day07 {
    const DAY: usize = 7;

    type Input = Vec<([usize; 5], usize)>;
    type ProcessedInput = Vec<([usize; 5], usize)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                (hand_values(hand), bid.parse().unwrap())
            })
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = score(&mut input, false);
        (input, ans.to_string())
    }

    fn solve_part2(mut input: Self::ProcessedInput) -> String {
        let joker_val = |n: usize| match n.cmp(&11) {
            Ordering::Greater => n,
            Ordering::Equal => 0,
            Ordering::Less => n + 1,
        };
        input
            .iter_mut()
            .for_each(|(h, _b)| (0..5).for_each(|i| h[i] = joker_val(h[i])));
        score(&mut input, true).to_string()
    }
}

#[cfg(test)]
mod test_day07 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_day07_examples() {
        let input = Day07::parse(EXAMPLE);
        let (input, part1) = Day07::solve_part1(input);
        let part2 = Day07::solve_part2(input);
        assert_eq!(part1, "6440");
        assert_eq!(part2, "5905");
    }
}
