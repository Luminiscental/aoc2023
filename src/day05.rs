use crate::day::Day;

struct SeedRange {
    ranges: Vec<(i64, i64)>,
}

impl SeedRange {
    fn min_loc(mut seeds: Vec<SeedRange>, maps: &[Vec<(i64, i64, i64)>]) -> i64 {
        seeds
            .iter_mut()
            .for_each(|s| maps.iter().for_each(|map| s.transform(map)));
        seeds
            .iter()
            .flat_map(|s| s.ranges.iter().map(|r| r.0))
            .min()
            .unwrap()
    }

    fn transform(&mut self, map: &[(i64, i64, i64)]) {
        let mut new_ranges = Vec::new();
        for (dest, source, len) in map.iter().copied() {
            let mut i = 0;
            while i < self.ranges.len() {
                let (start, end) = self.ranges[i];
                let new_start = source.max(start) + dest - source;
                let new_end = (source + len).min(end) + dest - source;
                if new_end > new_start {
                    self.ranges.swap_remove(i);
                    new_ranges.push((new_start, new_end));
                    if source > start {
                        self.ranges.push((start, source));
                    }
                    if end > source + len {
                        self.ranges.push((source + len, end));
                    }
                } else {
                    i += 1;
                }
            }
        }
        self.ranges.extend(new_ranges);
    }
}

pub struct Day05;

impl<'a> Day<'a> for Day05 {
    const DAY: usize = 5;

    type Input = (Vec<i64>, Vec<Vec<(i64, i64, i64)>>);
    type ProcessedInput = (Vec<i64>, Vec<Vec<(i64, i64, i64)>>);

    fn parse(input: &'a str) -> Self::Input {
        let mut pars = input.split("\n\n");
        let seeds = pars.next().unwrap();
        let (_, seeds) = seeds.split_once(':').unwrap();
        let seeds = seeds
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let maps = pars
            .map(|par| {
                par.lines()
                    .skip(1)
                    .map(|line| {
                        let mut ns = line.split_whitespace();
                        let mut eat = || ns.next().unwrap().parse().unwrap();
                        (eat(), eat(), eat())
                    })
                    .collect()
            })
            .collect();
        (seeds, maps)
    }

    fn solve_part1((seeds, maps): Self::Input) -> (Self::ProcessedInput, String) {
        let seed_ranges = seeds
            .iter()
            .map(|&s| SeedRange {
                ranges: vec![(s, s + 1)],
            })
            .collect::<Vec<_>>();
        let ans = SeedRange::min_loc(seed_ranges, &maps);
        ((seeds, maps), ans.to_string())
    }

    fn solve_part2((seeds, maps): Self::ProcessedInput) -> String {
        let seed_ranges = (0..seeds.len() / 2)
            .map(|i| SeedRange {
                ranges: vec![(seeds[2 * i], seeds[2 * i] + seeds[2 * i + 1])],
            })
            .collect::<Vec<_>>();
        SeedRange::min_loc(seed_ranges, &maps).to_string()
    }
}

#[cfg(test)]
mod test_day05 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_day05_examples() {
        let input = Day05::parse(EXAMPLE);
        let (input, part1) = Day05::solve_part1(input);
        let part2 = Day05::solve_part2(input);
        assert_eq!(part1, "35");
        assert_eq!(part2, "46");
    }
}
