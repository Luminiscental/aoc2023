use std::{array, collections::HashMap};

use crate::day::Day;

#[derive(Debug, Default, Clone)]
pub struct Workflow<'a> {
    filters: Vec<(usize, u64, bool, &'a str)>,
    default: &'a str,
}

fn count_accepted<'a>(
    name: &'a str,
    mut parts: [(u64, u64); 4],
    workflows: &HashMap<&'a str, Workflow<'a>>,
) -> u64 {
    if name == "R" || (0..4).any(|i| parts[i].1 < parts[i].0) {
        return 0;
    }
    if name == "A" {
        return (0..4).map(|i| parts[i].1 - parts[i].0 + 1).product();
    }
    let mut count = 0;
    let w = workflows.get(name).unwrap();
    for (i, val, less, target) in w.filters.iter().copied() {
        let mut enter = parts;
        if less {
            enter[i].1 = enter[i].1.min(val - 1);
            parts[i].0 = parts[i].0.max(val);
        } else {
            enter[i].0 = enter[i].0.max(val + 1);
            parts[i].1 = parts[i].1.min(val);
        }
        count += count_accepted(target, enter, workflows);
    }
    count + count_accepted(w.default, parts, workflows)
}

pub struct Day19;

impl<'a> Day<'a> for Day19 {
    const DAY: usize = 19;

    type Input = (HashMap<&'a str, Workflow<'a>>, Vec<[u64; 4]>);
    type ProcessedInput = HashMap<&'a str, Workflow<'a>>;

    fn parse(input: &'a str) -> Self::Input {
        let (workflows, parts) = input.trim().split_once("\n\n").unwrap();
        let workflows = workflows
            .lines()
            .map(|line| {
                let (name, workflow) = line.split_once('{').unwrap();
                let mut w = Workflow::default();
                for filter in workflow.split(',') {
                    if let Some((pred, target)) = filter.split_once(':') {
                        let less = pred.split_once('<').map(|(id, val)| (id, val, true));
                        let greater = pred.split_once('>').map(|(id, val)| (id, val, false));
                        let (id, val, is_less) = less.or(greater).unwrap();
                        let i = "xmas".bytes().position(|b| b == id.as_bytes()[0]).unwrap();
                        w.filters.push((i, val.parse().unwrap(), is_less, target));
                    } else {
                        w.default = &filter[..filter.len() - 1];
                        break;
                    }
                }
                (name, w)
            })
            .collect();
        let parts = parts
            .lines()
            .map(|line| {
                let mut vals = line.split(',').map(|p| {
                    let s = p.find(|c: char| c.is_ascii_digit()).unwrap();
                    let l = p[s..]
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(p[s..].len());
                    p[s..s + l].parse().unwrap()
                });
                array::from_fn(|_| vals.next().unwrap())
            })
            .collect();
        (workflows, parts)
    }

    fn solve_part1((workflows, parts): Self::Input) -> (Self::ProcessedInput, String) {
        let ans = parts
            .iter()
            .filter(|p| count_accepted("in", array::from_fn(|i| (p[i], p[i])), &workflows) > 0)
            .flat_map(|p| p.iter())
            .sum::<u64>();
        (workflows, ans.to_string())
    }

    fn solve_part2(workflows: Self::ProcessedInput) -> String {
        count_accepted("in", [(1, 4000); 4], &workflows).to_string()
    }
}

#[cfg(test)]
mod test_day19 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_day19_examples() {
        let input = Day19::parse(EXAMPLE);
        let (input, part1) = Day19::solve_part1(input);
        let part2 = Day19::solve_part2(input);
        assert_eq!(part1, "19114");
        assert_eq!(part2, "167409079868000");
    }
}
