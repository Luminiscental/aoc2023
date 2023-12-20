use std::collections::{HashMap, VecDeque};

use num::integer;

use crate::day::Day;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mod {
    #[default]
    Conj,
    Flip(bool),
}

#[derive(Default, Debug, Clone)]
pub struct Node<'a> {
    ins: Vec<&'a str>,
    outs: Vec<&'a str>,
    last: bool,
    ty: Mod,
}

#[derive(Default, Debug, Clone)]
pub struct Circuit<'a> {
    broadcasts: Vec<&'a str>,
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Circuit<'a> {
    fn broadcast(&mut self, target: &'a str) -> ([usize; 2], bool) {
        let mut queue = self
            .broadcasts
            .iter()
            .map(|&b| (b, false))
            .collect::<VecDeque<_>>();
        let mut pulses = [1, 0];
        let mut hit = false;
        while let Some((n, high)) = queue.pop_front() {
            pulses[high as usize] += 1;
            let node = self.nodes.get_mut(n).unwrap();
            let mut pulse = None;
            match &mut node.ty {
                Mod::Flip(flip) if !high => {
                    *flip = !*flip;
                    pulse = Some(*flip);
                }
                Mod::Conj => {
                    let node = self.nodes.get(n).unwrap();
                    pulse = Some(!node.ins.iter().all(|&i| self.nodes.get(i).unwrap().last));
                }
                _ => (),
            }
            if let Some(high) = pulse {
                if (n, high) == (target, true) {
                    hit = true;
                }
                let node = self.nodes.get_mut(n).unwrap();
                node.last = high;
                queue.extend(node.outs.iter().map(|&o| (o, high)));
            }
        }
        (pulses, hit)
    }
}

pub struct Day20;

impl<'a> Day<'a> for Day20 {
    const DAY: usize = 20;

    type Input = Circuit<'a>;
    type ProcessedInput = Circuit<'a>;

    fn parse(input: &'a str) -> Self::Input {
        let mut circuit = Circuit::default();
        for line in input.trim().lines() {
            let (input, output) = line.split_once(" -> ").unwrap();
            let outs = output.split(", ").collect::<Vec<_>>();
            if input == "broadcaster" {
                circuit.broadcasts = outs;
            } else {
                let (module, input) = input.split_at(1);
                outs.iter()
                    .for_each(|out| circuit.nodes.entry(out).or_default().ins.push(input));
                let node = circuit.nodes.entry(input).or_default();
                if module == "%" {
                    node.ty = Mod::Flip(false);
                }
                node.outs = outs;
            }
        }
        circuit
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut circuit = input.clone();
        let mut pulses = [0; 2];
        for _ in 0..1000 {
            let p = circuit.broadcast("").0;
            (0..2).for_each(|i| pulses[i] += p[i]);
        }
        let ans = pulses[0] * pulses[1];
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let rx = input.nodes.get("rx").unwrap();
        assert_eq!(rx.ins.len(), 1);
        let trigger = input.nodes.get(rx.ins[0]).unwrap();
        assert_eq!(trigger.ty, Mod::Conj);
        let mut periods = vec![];
        for n in trigger.ins.iter().copied() {
            let mut circuit = input.clone();
            periods.push((1..).find(|_| circuit.broadcast(n).1).unwrap());
        }
        periods.into_iter().fold(1u64, integer::lcm).to_string()
    }
}

#[cfg(test)]
mod test_day20 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    #[test]
    fn test_day20_examples() {
        let input = Day20::parse(EXAMPLE);
        let (_, part1) = Day20::solve_part1(input);
        assert_eq!(part1, "32000000");
    }
}
