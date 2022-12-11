use crate::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Display, FromStr, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(i64),
}

#[derive(Copy, Clone, Debug)]
struct CPUState {
    x: i64,
}

struct CPU {
    next_state: CPUState,
    current_op: usize,
    waiting: usize,
    ops: Vec<Operation>,
}

impl Iterator for CPU {
    type Item = CPUState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_op >= self.ops.len() {
            return None;
        }

        match (self.waiting, self.ops[self.current_op]) {
            (0, Operation::Addx(_)) => {
                self.waiting = 1;
                Some(self.next_state)
            }
            (1, Operation::Addx(offset)) => {
                self.waiting -= 1;
                self.current_op += 1;
                let last = self.next_state;
                self.next_state.x += offset;
                Some(last)
            }
            (0, Operation::Noop) => {
                self.current_op += 1;
                Some(self.next_state)
            }
            _ => unreachable!(),
        }
    }
}

fn cpu_iter(ops: &[Operation]) -> impl Iterator<Item = CPUState> {
    CPU {
        next_state: CPUState { x: 1 },
        current_op: 0,
        waiting: 0,
        ops: ops.to_vec(),
    }
}

/// https://adventofcode.com/2022/day/10
#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Operation> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Operation]) -> i64 {
    cpu_iter(input)
        .enumerate()
        .map(|(a, b)| (a + 1, b))
        .filter(|(idx, _)| match *idx {
            20 | 60 | 100 | 140 | 180 | 220 => true,
            _ => false,
        })
        .map(|(idx, CPUState { x })| idx as i64 * x)
        .sum::<i64>()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Operation]) -> String {
    "\n".to_string()
        + &cpu_iter(input)
            .enumerate()
            .map(|(a, b)| (a + 1, b))
            .map(|(cycle, cpu)| [cpu.x - 1, cpu.x, cpu.x + 1].contains(&((cycle as i64 - 1) % 40)))
            .map(|draw| match draw {
                true => '█',
                false => ' ',
            })
            .collect_vec()
            .array_chunks::<40>()
            .map(|x| x.into_iter().collect::<String>())
            .collect_vec()
            .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        assert_eq!(13140, solve_part1(&generator(EXAMPLE_INPUT)));
        assert_eq!(
            13220,
            solve_part1(&generator(include_str!("../input/2022/day10.txt")))
        );
    }

    // #[test]
    // fn it_works_part2() {
    // assert_eq!(EXAMPLE_INPUT_SOL, solve_part2(&generator(EXAMPLE_INPUT)));
    // assert_eq!(
    //     385112,
    //     solve_part2(&generator(include_str!("../input/2022/day8.txt")))
    // );
    // }

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}

const EXAMPLE_INPUT_SOL: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
