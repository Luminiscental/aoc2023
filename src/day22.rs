use std::{
    array,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::iproduct;

use crate::day::Day;

fn brick_range(b: ([usize; 3], [usize; 3])) -> impl Iterator<Item = [usize; 3]> {
    iproduct!(b.0[0]..=b.1[0], b.0[1]..=b.1[1], b.0[2]..=b.1[2]).map(|(x, y, z)| [x, y, z])
}

fn count_dropped(
    brick: usize,
    resters: &HashMap<usize, HashSet<usize>>,
    supporters: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut dropped = HashSet::<usize>::new();
    let mut queue = VecDeque::new();
    queue.push_back(brick);
    while let Some(i) = queue.pop_front() {
        dropped.insert(i);
        if let Some(rs) = resters.get(&i) {
            queue.extend(rs.iter().filter(|r| {
                supporters
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
    type ProcessedInput = (
        usize,
        HashMap<usize, HashSet<usize>>,
        HashMap<usize, HashSet<usize>>,
    );

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

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut bricks = input.clone();
        let mut space = bricks
            .iter()
            .enumerate()
            .flat_map(|(i, &b)| brick_range(b).map(move |p| (p, i)))
            .collect::<HashMap<_, _>>();
        let lower = |mut b: ([usize; 3], [usize; 3]), d: usize| {
            b.0[2] -= d;
            b.1[2] -= d;
            b
        };
        let mut moved = true;
        while moved {
            moved = false;
            for (i, b) in bricks.iter_mut().enumerate() {
                let mut scan = *b;
                scan.1[2] = scan.0[2];
                if let Some(j) = (1..b.0[2])
                    .take_while(|&j| brick_range(lower(scan, j)).all(|p| !space.contains_key(&p)))
                    .last()
                {
                    for p in brick_range(*b) {
                        space.remove(&p);
                    }
                    *b = lower(*b, j);
                    for p in brick_range(*b) {
                        space.insert(p, i);
                    }
                    moved = true;
                }
            }
        }
        let mut resters = HashMap::<usize, HashSet<usize>>::new();
        let mut supporters = HashMap::<usize, HashSet<usize>>::new();
        for (i, &b) in bricks.iter().enumerate() {
            let vert = b.0[2] != b.1[2];
            let mut scan = lower(b, 1);
            if vert {
                scan = (scan.0, scan.0);
            }
            for p in brick_range(scan) {
                if let Some(j) = space.get(&p).copied() {
                    resters.entry(j).or_default().insert(i);
                    supporters.entry(i).or_default().insert(j);
                }
            }
        }
        let ans = (0..bricks.len())
            .filter(|i| {
                resters.get(i).map_or(true, |r| {
                    r.iter().all(|j| supporters.get(j).unwrap().len() > 1)
                })
            })
            .count();
        ((bricks.len(), resters, supporters), ans.to_string())
    }

    fn solve_part2((bricks, resters, supporters): Self::ProcessedInput) -> String {
        (0..bricks)
            .map(|i| count_dropped(i, &resters, &supporters))
            .sum::<usize>()
            .to_string()
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
