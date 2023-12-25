use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{day::Day, util::Ignore};

fn count_edge_occurences<'a>(
    fwd: &HashMap<&'a str, Vec<&'a str>>,
    rev: &HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<(&'a str, &'a str), usize> {
    let mut occ = HashMap::new();
    for &start in fwd.keys() {
        let (mut queue, mut seen) = (VecDeque::new(), HashSet::new());
        queue.push_back(start);
        seen.insert(start);
        while let Some(n) = queue.pop_front() {
            let nxt = fwd[n].iter().map(|&m| (m, (n, m)));
            let prev = rev[n].iter().map(|&m| (m, (m, n)));
            for (m, e) in nxt.chain(prev) {
                if seen.insert(m) {
                    *occ.entry(e).or_default() += 1;
                    queue.push_back(m);
                }
            }
        }
    }
    occ
}

pub struct Day25;

impl<'a> Day<'a> for Day25 {
    const DAY: usize = 25;

    type Input = HashMap<&'a str, Vec<&'a str>>;
    type ProcessedInput = ();

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (k, vs) = line.split_once(": ").unwrap();
                (k, vs.split_whitespace().collect())
            })
            .collect()
    }

    fn solve_part1(mut fwd: Self::Input) -> (Self::ProcessedInput, String) {
        let (fwd, rev) = {
            let mut rev = HashMap::<&'a str, Vec<&'a str>>::new();
            fwd.iter()
                .for_each(|(k, vs)| vs.iter().for_each(|v| rev.entry(*v).or_default().push(*k)));
            rev.keys().for_each(|k| fwd.entry(k).or_default().ignore());
            fwd.keys().for_each(|k| rev.entry(k).or_default().ignore());
            (fwd, rev)
        };
        let occ = count_edge_occurences(&fwd, &rev);
        let edges = {
            let mut edges = fwd
                .iter()
                .flat_map(|(&k, vs)| vs.iter().map(move |&v| (k, v)))
                .collect::<Vec<_>>();
            edges.sort_unstable_by_key(|e| occ[e]);
            edges.reverse();
            edges
        };
        for forbid in edges.into_iter().combinations(3) {
            let start = fwd.keys().copied().next().unwrap();
            let mut queue = vec![start];
            let mut seen = queue.iter().copied().collect::<HashSet<_>>();
            while let Some(k) = queue.pop() {
                let nxt = fwd[k].iter().filter(|e| !forbid.contains(&(k, e)));
                let prev = rev[k].iter().filter(|e| !forbid.contains(&(e, k)));
                let l = queue.len();
                queue.extend(nxt.chain(prev).filter(|&e| !seen.contains(e)));
                seen.extend(queue.iter().copied().skip(l));
            }
            if seen.len() < fwd.len() {
                let ans = seen.len() * (fwd.len() - seen.len());
                return ((), ans.to_string());
            }
        }
        panic!("no cut found")
    }

    fn solve_part2(_: Self::ProcessedInput) -> String {
        "Merry Christmas!".to_string()
    }
}

#[cfg(test)]
mod test_day25 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn test_day25_examples() {
        let input = Day25::parse(EXAMPLE);
        let (_, part1) = Day25::solve_part1(input);
        assert_eq!(part1, "54");
    }
}
