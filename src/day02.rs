use crate::day::Day;

const COLORS: [&str; 3] = ["red", "green", "blue"];

fn three_max(a: [u32; 3], b: [u32; 3]) -> [u32; 3] {
    let mut max = [0; 3];
    (0..3).for_each(|i| max[i] = u32::max(a[i], b[i]));
    max
}

fn three_le(a: [u32; 3], b: [u32; 3]) -> bool {
    (0..3).all(|i| a[i] <= b[i])
}

pub struct Day02;

impl<'a> Day<'a> for Day02 {
    const DAY: usize = 2;

    type Input = Vec<(u32, Vec<[u32; 3]>)>;
    type ProcessedInput = Vec<(u32, Vec<[u32; 3]>)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (idx, content) = line.split_once(':').unwrap();
                let game_idx = idx.split(' ').last().unwrap().parse().unwrap();
                let reveals = content
                    .split(';')
                    .map(|reveal| {
                        let mut shown = [0; 3];
                        reveal.split(',').for_each(|amount| {
                            let (number, color) = amount.trim().split_once(' ').unwrap();
                            let color_idx = COLORS.iter().position(|&c| c == color).unwrap();
                            shown[color_idx] = number.parse().unwrap();
                        });
                        shown
                    })
                    .collect();
                (game_idx, reveals)
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let amounts = [12, 13, 14];
        let ans = input
            .iter()
            .filter(|&(_id, reveals)| reveals.iter().all(|&reveal| three_le(reveal, amounts)))
            .map(|(id, _reveals)| id)
            .sum::<u32>();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .into_iter()
            .map(|(_id, reveals)| {
                let max = reveals.into_iter().fold([0; 3], three_max);
                max[0] * max[1] * max[2]
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day02 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_day02_examples() {
        let input = Day02::parse(EXAMPLE);
        let (input, part1) = Day02::solve_part1(input);
        let part2 = Day02::solve_part2(input);
        assert_eq!(part1, "8");
        assert_eq!(part2, "2286");
    }
}
