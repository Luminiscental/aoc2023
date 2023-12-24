use std::array;

use crate::day::Day;

fn gauss<const N: usize, const M: usize>(mat: &mut [[f64; N]; M]) {
    let (mut pr, mut pc) = (0, 0);
    while pr < M && pc < N {
        let i_max = (pr..M).max_by_key(|&i| mat[i][pc].abs() as i64).unwrap();
        if mat[i_max][pc].abs() < 0.000001 {
            pc += 1;
            continue;
        }
        mat.swap(pr, i_max);
        for i in pr + 1..M {
            let f = mat[i][pc] / mat[pr][pc];
            mat[i][pc] = 0.0;
            for j in pc + 1..N {
                mat[i][j] -= mat[pr][j] * f;
            }
        }
        (pr, pc) = (pr + 1, pc + 1);
    }
}

pub struct Day24Generic<const MIN: i64, const MAX: i64>;
pub type Day24 = Day24Generic<200000000000000, 400000000000000>;

impl<'a, const MIN: i64, const MAX: i64> Day<'a> for Day24Generic<MIN, MAX> {
    const DAY: usize = 24;

    type Input = Vec<([f64; 3], [f64; 3])>;
    type ProcessedInput = Vec<([f64; 3], [f64; 3])>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_once(" @ ").unwrap();
                let split = |s: &'a str| {
                    let mut it = s.split(", ").map(|n| n.trim().parse().unwrap());
                    move |_| it.next().unwrap()
                };
                (array::from_fn(split(pos)), array::from_fn(split(vel)))
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut ans = 0;
        for (i, ([x0, y0, _], [dx0, dy0, _])) in input.iter().copied().enumerate() {
            for ([x1, y1, _], [dx1, dy1, _]) in input.iter().copied().take(i) {
                let disc = dx0 * dy1 - dx1 * dy0;
                if disc == 0.0 {
                    continue;
                }
                let s = (y0 * dx0 - x0 * dy0 + x1 * dy0 - y1 * dx0) / disc;
                let t = (x1 * dy1 - y1 * dx1 + y0 * dx1 - x0 * dy1) / disc;
                if s < 0.0 || t < 0.0 {
                    continue;
                }
                let (x, y) = (x1 + s * dx1, y1 + s * dy1);
                let (min, max) = (MIN as f64, MAX as f64);
                ans += (min <= x && x <= max && min <= y && y <= max) as usize;
            }
        }
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let eqn = |i| {
            let ([x0, y0, z0], [dx0, dy0, dz0]) = input[0];
            let ([x1, y1, z1], [dx1, dy1, dz1]) = input[i];
            let rx1 = (y0 * dz0 - z0 * dy0) - (y1 * dz1 - z1 * dy1);
            let ry1 = (z0 * dx0 - x0 * dz0) - (z1 * dx1 - x1 * dz1);
            let rz1 = (x0 * dy0 - y0 * dx0) - (x1 * dy1 - y1 * dx1);
            [
                [0.0, dz0 - dz1, dy1 - dy0, 0.0, z1 - z0, y0 - y1, rx1],
                [dz1 - dz0, 0.0, dx0 - dx1, z0 - z1, 0.0, x1 - x0, ry1],
                [dy0 - dy1, dx1 - dx0, 0.0, y1 - y0, x0 - x1, 0.0, rz1],
            ]
        };
        let [e1, e2, e3] = eqn(1);
        let [e4, e5, e6] = eqn(2);
        let mut aug = [e1, e2, e3, e4, e5, e6];
        gauss(&mut aug);
        let mut soln = [0.0; 6];
        for i in (0..6).rev() {
            soln[i] = (aug[i][6] - (i..6).map(|j| aug[i][j] * soln[j]).sum::<f64>()) / aug[i][i];
        }
        let [x, y, z] = array::from_fn(|i| soln[i].round() as i64);
        (x + y + z).to_string()
    }
}

#[cfg(test)]
mod test_day24 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_day24_examples() {
        let input = Day24Generic::<7, 27>::parse(EXAMPLE);
        let (input, part1) = Day24Generic::<7, 27>::solve_part1(input);
        let part2 = Day24::solve_part2(input);
        assert_eq!(part1, "2");
        assert_eq!(part2, "47");
    }
}
