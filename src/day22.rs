use std::{
    array,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::iproduct;

use crate::day::Day;

#[derive(Default)]
pub struct Support {
    up: HashMap<usize, HashSet<usize>>,
    down: HashMap<usize, HashSet<usize>>,
}

fn range(b: ([usize; 3], [usize; 3])) -> impl Iterator<Item = [usize; 3]> {
    iproduct!(b.0[0]..=b.1[0], b.0[1]..=b.1[1], b.0[2]..=b.1[2]).map(|(x, y, z)| [x, y, z])
}

fn lower(b: ([usize; 3], [usize; 3]), d: usize) -> ([usize; 3], [usize; 3]) {
    ([b.0[0], b.0[1], b.0[2] - d], [b.1[0], b.1[1], b.1[2] - d])
}

fn settle_bricks(bricks: &mut [([usize; 3], [usize; 3])]) -> HashMap<[usize; 3], usize> {
    let mut space = bricks
        .iter()
        .enumerate()
        .flat_map(|(i, &b)| range(b).map(move |p| (p, i)))
        .collect::<HashMap<_, _>>();
    let mut moved = true;
    while moved {
        moved = false;
        for (i, b) in bricks.iter_mut().enumerate() {
            let mut scan = *b;
            scan.1[2] = scan.0[2];
            if let Some(j) = (1..b.0[2])
                .take_while(|&j| range(lower(scan, j)).all(|p| !space.contains_key(&p)))
                .last()
            {
                for p in range(*b) {
                    space.remove(&p);
                }
                *b = lower(*b, j);
                space.extend(range(*b).map(|p| (p, i)));
                moved = true;
            }
        }
    }
    space
}

fn get_support(bricks: &[([usize; 3], [usize; 3])], space: &HashMap<[usize; 3], usize>) -> Support {
    let mut support = Support::default();
    for (i, &b) in bricks.iter().enumerate() {
        let mut scan = lower(b, 1);
        scan.1[2] = scan.0[2];
        for p in range(scan) {
            if let Some(j) = space.get(&p).copied() {
                support.up.entry(j).or_default().insert(i);
                support.down.entry(i).or_default().insert(j);
            }
        }
    }
    support
}

fn count_dropped(brick: usize, support: &Support) -> usize {
    let (mut dropped, mut queue) = (HashSet::new(), VecDeque::new());
    queue.push_back(brick);
    while let Some(i) = queue.pop_front() {
        dropped.insert(i);
        if let Some(rs) = support.up.get(&i) {
            queue.extend(rs.iter().filter(|r| {
                support
                    .down
                    .get(r)
                    .unwrap()
                    .iter()
                    .all(|s| dropped.contains(s))
            }));
        }
    }
    dropped.len() - 1
}

pub struct Day22;

impl<'a> Day<'a> for Day22 {
    const DAY: usize = 22;

    type Input = Vec<([usize; 3], [usize; 3])>;
    type ProcessedInput = Vec<usize>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (s, e) = line.split_once('~').unwrap();
                let split = |s: &'a str| {
                    let mut it = s.split(',').map(|n| n.parse().unwrap());
                    move |_| it.next().unwrap()
                };
                (array::from_fn(split(s)), array::from_fn(split(e)))
            })
            .collect()
    }

    fn solve_part1(mut bricks: Self::Input) -> (Self::ProcessedInput, String) {
        let space = settle_bricks(&mut bricks);
        let support = get_support(&bricks, &space);
        let counts = (0..bricks.len())
            .map(|i| count_dropped(i, &support))
            .collect::<Vec<_>>();
        let ans = counts.iter().filter(|n| **n == 0).count();
        (counts, ans.to_string())
    }

    fn solve_part2(counts: Self::ProcessedInput) -> String {
        counts.into_iter().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod test_day22 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_day22_examples() {
        let input = Day22::parse(EXAMPLE);
        let (input, part1) = Day22::solve_part1(input);
        let part2 = Day22::solve_part2(input);
        assert_eq!(part1, "5");
        assert_eq!(part2, "7");
    }
}
