use crate::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Display, FromStr, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Display, FromStr, Debug, Copy, Clone, Eq, PartialEq)]
#[display("{direction} {magnitude}")]
pub struct Instruction {
    direction: Direction,
    magnitude: isize,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn is_adjacent(&self, tail: &Point) -> bool {
        (self.x - tail.x).abs() <= 1 && (self.y - tail.y).abs() <= 1
    }

    fn goto_target(&mut self, target: &Point) {
        let x_diff = (target.x - self.x);
        let y_diff = (target.y - self.y);
        match (x_diff.abs(), y_diff.abs()) {
            (2, 0) => {
                // two to the right or left
                self.x += x_diff / x_diff.abs();
            }
            (0, 2) => {
                // two above or below
                self.y += y_diff / y_diff.abs()
            }
            (2, 1) | (1, 2) | (2,2) => {
                // two in one direction, one in the other -- needs to move diagonally
                self.y += y_diff / y_diff.abs();
                self.x += x_diff / x_diff.abs();
            }
            _ => {
                println!("diff {} {}", x_diff, y_diff);
                unreachable!()
            },
        }
    }
}

struct Board {
    knots: Vec<Point>,
    visited: HashSet<Point>
}

// impl Display for Board {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let 
//         for i in 0..5 {
//             for j in 0..5 {
//                 if i == (4-self.head.y) % 5 && j == self.head.x % 5 {
//                     write!(f, "H")?;
//                 } else if i == (4-self.tail.y) % 5 && j == self.tail.x % 5 {
//                     write!(f, "T")?;
//                 } else {
//                     write!(f, ".")?;
//                 }
//             }
//             write!(f, "\n")?;
//         }
// 
//         Ok(())
//     }
// }

impl Board {
    fn new(knots: usize) -> Self {
        Board {
            knots: (0..knots).map(|_| Point { x: 0, y: 0 }).collect(),
            visited: [Point { x:0, y:0}].into_iter().collect()
        }
    }

    fn apply(&mut self, instr: &Instruction) {

        for _ in 0..instr.magnitude {
            self.knots[0].step(instr.direction);


            for idx in 0..self.knots.len()-1 {
                let head = self.knots[idx].clone();
                let tail = &mut self.knots[idx+1];
                if !head.is_adjacent(tail) {
                    tail.goto_target(&head);
                }
            }
            self.visited.insert(self.knots.last().unwrap().clone());

        }
    }
}

/// https://adventofcode.com/2022/day/9
#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Instruction]) -> usize {
    let mut board = Board::new(2);
    for i in input {
        board.apply(i)
    }
    board.visited.len()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Instruction]) -> usize {
    let mut board = Board::new(10);
    for i in input {
        board.apply(i)
    }
    board.visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, solve_part1(&generator(provided)));
        assert_eq!(
            5981,
            solve_part1(&generator(include_str!("../input/2022/day9.txt")))
        );
    }

        #[test]
        fn it_works_part2() {
            let provided = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
            assert_eq!(36, solve_part2(&generator(provided)));
            // assert_eq!(
            //     385112,
            //     solve_part2(&generator(include_str!("../input/2022/day8.txt")))
            // );
        }
}
