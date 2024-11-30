// https://adventofcode.com/2023/day/24

use crate::{
    grids::FPoint,
    solution::{InputType, Solution},
};
use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

type Point3d = (isize, isize, isize);

pub struct Day24 {
    lines: Vec<(Point3d, Point3d)>,
}

impl<'i> Solution<'i> for Day24 {
    type Part1Output = usize;
    type Part2Output = u64;

    fn title(&self) -> &str {
        "Never Tell Me The Odds"
    }

    fn parse(input: &'i str) -> Self {
        let lines = input
            .split('\n')
            .map(|line| {
                let (point, velocity) = line.split_once(" @ ").unwrap();
                let coords = point
                    .split(", ")
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                let point = (coords[0], coords[1], coords[2]);
                let coords = velocity
                    .split(", ")
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                let velocity = (coords[0], coords[1], coords[2]);
                (point, velocity)
            })
            .collect::<Vec<(Point3d, Point3d)>>();
        Self { lines }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let interval = if self.lines.len() == 5 {
            (7., 27.)
        } else {
            (200000000000000., 400000000000000.)
        };
        let lines2d = self
            .lines
            .iter()
            .map(|(p, v)| {
                (
                    FPoint::new(p.0 as f64, p.1 as f64),
                    FPoint::new(v.0 as f64, v.1 as f64),
                )
            })
            .collect::<Vec<(FPoint, FPoint)>>();
        lines2d
            .iter()
            .tuple_combinations()
            .filter(|((p1, v1), (p2, v2))| {
                let a1 = v1.y / v1.x;
                let a2 = v2.y / v2.x;
                let b1 = p1.y - a1 * p1.x;
                let b2 = p2.y - a2 * p2.x;

                let x = (b2 - b1) / (a1 - a2);
                let y = a1 * x + b1;
                interval.0 <= x
                    && x <= interval.1
                    && interval.0 <= y
                    && y <= interval.1
                    && if v1.x < 0. { x <= p1.x } else { x >= p1.x }
                    && if v1.y < 0. { y <= p1.y } else { y >= p1.y }
                    && if v2.x < 0. { x <= p2.x } else { x >= p2.x }
                    && if v2.y < 0. { y <= p2.y } else { y >= p2.y }
            })
            .count()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        // Probably the least satisfying AoC problem I've ever done.
        // Bruteforcing does not seem feasible at all, and cannot
        // think of any solution except having it solved for me.
        // There might be some assumption that can be made about
        // the input that allows for a better solution, but after
        // not finding one myself and scrolling through Reddit
        // this is the only consistent solution I could find.
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let rock_px = Int::new_const(&ctx, "px");
        let rock_py = Int::new_const(&ctx, "py");
        let rock_pz = Int::new_const(&ctx, "pz");
        let rock_vx = Int::new_const(&ctx, "vx");
        let rock_vy = Int::new_const(&ctx, "vy");
        let rock_vz = Int::new_const(&ctx, "vz");

        self.lines.iter().take(3).for_each(|(hail_p, hail_v)| {
            let hail_px = Int::from_i64(&ctx, hail_p.0 as i64);
            let hail_py = Int::from_i64(&ctx, hail_p.1 as i64);
            let hail_pz = Int::from_i64(&ctx, hail_p.2 as i64);
            let hail_vx = Int::from_i64(&ctx, hail_v.0 as i64);
            let hail_vy = Int::from_i64(&ctx, hail_v.1 as i64);
            let hail_vz = Int::from_i64(&ctx, hail_v.2 as i64);
            let time = Int::fresh_const(&ctx, "t");

            solver.assert(&(&hail_px + &hail_vx * &time)._eq(&(&rock_px + &rock_vx * &time)));
            solver.assert(&(&hail_py + &hail_vy * &time)._eq(&(&rock_py + &rock_vy * &time)));
            solver.assert(&(&hail_pz + &hail_vz * &time)._eq(&(&rock_pz + &rock_vz * &time)));
        });
        if solver.check() == SatResult::Sat {
            if let Some(model) = solver.get_model() {
                return model
                    .eval(&(rock_px + rock_py + rock_pz), true)
                    .unwrap()
                    .as_u64()
                    .unwrap();
            }
        }
        0
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(2), Some(47)),
            InputType::Puzzles => (Some(15889), Some(801386475216902)),
        }
    }
}
