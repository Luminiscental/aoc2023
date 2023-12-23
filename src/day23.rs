use std::collections::HashMap;

use crate::{day::Day, util::LineGrid};

fn graph_of(grid: &LineGrid<'_>, key: bool) -> Vec<Vec<(usize, u32)>> {
    let mut nodes = vec![(0, 1), (grid.height() as i32 - 1, grid.width() as i32 - 2)];
    let mut graph = vec![Vec::new(); 2];
    let mut seen = 1u64;
    let mut queue = vec![((0, 1), (0, 1), 0, 0)];
    while let Some((curr, prev, root, steps)) = queue.pop() {
        let mut nxts = Vec::new();
        for (d, gate) in [((-1, 0), '^'), ((0, -1), '<'), ((1, 0), 'v'), ((0, 1), '>')] {
            let nxt = (curr.0 + d.0, curr.1 + d.1);
            if nxt == prev {
                continue;
            }
            match grid.try_get(nxt.0, nxt.1) {
                Some(c) if c == '.' || (key && "^<v>".contains(c)) => nxts.push((nxt, 1)),
                Some(c) if c == gate => nxts.push(((nxt.0 + d.0, nxt.1 + d.1), 2)),
                _ => (),
            }
        }
        if nxts.len() == 1 {
            queue.push((nxts[0].0, curr, root, steps + nxts[0].1))
        } else {
            let node = nodes.iter().position(|&m| m == curr).unwrap_or_else(|| {
                nodes.push(curr);
                graph.push(Vec::new());
                nodes.len() - 1
            });
            graph[root].push((node, steps));
            if key {
                graph[node].push((root, steps));
            }
            if seen & (1 << node) == 0 {
                seen |= 1 << node;
                queue.extend(nxts.into_iter().map(|(nxt, s)| (nxt, curr, node, s)));
            }
        }
    }
    graph
}

fn max_path(
    graph: &[Vec<(usize, u32)>],
    curr: usize,
    path: u64,
    memo: &mut HashMap<(usize, u64), Option<u32>>,
) -> Option<u32> {
    if curr == 1 {
        return Some(0);
    }
    memo.get(&(curr, path)).copied().unwrap_or_else(|| {
        let steps = graph[curr]
            .iter()
            .filter(|(nxt, _)| path & (1 << nxt) == 0)
            .filter_map(|&(nxt, steps)| {
                max_path(graph, nxt, path | (1 << nxt), memo).map(|l| l + steps)
            })
            .fold(0, u32::max);
        let ans = (steps > 0).then_some(steps);
        memo.insert((curr, path), ans);
        ans
    })
}

pub struct Day23;

impl<'a> Day<'a> for Day23 {
    const DAY: usize = 23;

    type Input = LineGrid<'a>;
    type ProcessedInput = LineGrid<'a>;

    fn parse(input: &'a str) -> Self::Input {
        LineGrid::new(input)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = max_path(&graph_of(&input, false), 0, 1, &mut HashMap::new()).unwrap();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        max_path(&graph_of(&input, true), 0, 1, &mut HashMap::new())
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day23 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn test_day23_examples() {
        let input = Day23::parse(EXAMPLE);
        let (input, part1) = Day23::solve_part1(input);
        let part2 = Day23::solve_part2(input);
        assert_eq!(part1, "94");
        assert_eq!(part2, "154");
    }
}
