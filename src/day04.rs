use std::collections::HashMap;

use crate::day::Day;

pub struct Day04;

impl<'a> Day<'a> for Day04 {
    const DAY: usize = 4;

    type Input = HashMap<usize, (Vec<u32>, Vec<u32>)>;
    type ProcessedInput = HashMap<usize, usize>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (card_id, content) = line.split_once(':').unwrap();
                let (_card, id) = card_id.trim().split_once(' ').unwrap();
                let (win, have) = content.trim().split_once('|').unwrap();
                let get_nums = |s: &str| s.split_whitespace().map(|n| n.parse().unwrap()).collect();
                (id.trim().parse().unwrap(), (get_nums(win), get_nums(have)))
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let wins: HashMap<_, _> = input
            .into_iter()
            .map(|(card, (win, have))| (card, have.iter().filter(|n| win.contains(n)).count()))
            .collect();
        let ans = wins
            .values()
            .map(|&n| (n > 0).then(|| 1 << (n - 1)).unwrap_or(0))
            .sum::<usize>();
        (wins, ans.to_string())
    }

    fn solve_part2(wins: Self::ProcessedInput) -> String {
        let mut copies: HashMap<usize, usize> = wins.keys().map(|&c| (c, 1)).collect();
        let mut done: HashMap<usize, usize> = HashMap::new();
        while !copies.is_empty() {
            let (&card, &n) = copies.iter().next().unwrap();
            let count = wins.get(&card).unwrap();
            (card + 1..=card + count).for_each(|c| *copies.entry(c).or_insert(0) += n);
            *done.entry(card).or_insert(0) += n;
            copies.remove(&card);
        }
        done.values().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod test_day04 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_day04_examples() {
        let input = Day04::parse(EXAMPLE);
        let (input, part1) = Day04::solve_part1(input);
        let part2 = Day04::solve_part2(input);
        assert_eq!(part1, "13");
        assert_eq!(part2, "30");
    }
}
