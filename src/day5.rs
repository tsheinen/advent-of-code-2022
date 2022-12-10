use crate::prelude::*;
use std::str::FromStr;

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Instruction {
    #[display("move {0} from {1} to {2}")]
    Move(usize, usize, usize),
}

#[derive(Debug, Clone)]
pub struct Stacks {
    rows: Vec<Vec<u8>>,
}

impl Stacks {
    fn apply_part_one(mut self, instr: &Instruction) -> Self {
        match instr {
            Instruction::Move(amount, src, dest) => {
                for _ in 0..*amount {
                    let elem = self.rows[*src - 1]
                        .pop()
                        .expect("trying to move more elements than exist");
                    self.rows[*dest - 1].push(elem);
                }
                self
            }
        }
    }

    fn apply_part_two(mut self, instr: &Instruction) -> Self {
        match instr {
            Instruction::Move(amount, src, dest) => {
                let src_vec_len = self.rows[*src - 1].len();
                let copied = self.rows[*src - 1]
                    .drain(src_vec_len - *amount..src_vec_len)
                    .collect_vec();
                for elem in copied {
                    self.rows[*dest - 1].push(elem);
                }
                self
            }
        }
    }
    
    fn get_top(&self) -> String {
        self
            .rows
            .iter()
            .filter_map(|x| x.last())
            .map(|x| *x as char)
            .collect()
    }
    
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
            assert!(!v.is_empty());
            let len = v[0].len();
            let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
            (0..len)
                .map(|_| {
                    iters
                        .iter_mut()
                        .map(|n| n.next().unwrap())
                        .rev()
                        .collect::<Vec<T>>()
                })
                .collect()
        }

        let mut rows = vec![];
        let num_lines = s.as_bytes().iter().filter(|x| **x == b'\n').count();
        for i in s.split_inclusive('\n').take(num_lines) {
            let mut row = vec![];
            for chunk in i.as_bytes().array_chunks::<4>() {
                if chunk.iter().take(3).all(|x| *x == b' ') {
                    row.push(None)
                } else {
                    row.push(Some(chunk[1]))
                }
            }
            rows.push(row);
        }
        let rows = transpose(rows);
        let rows = rows
            .iter()
            .map(|row| row.iter().filter_map(|x| *x).collect_vec())
            .collect_vec();
        Ok(Stacks { rows: rows })
    }
}

/// https://adventofcode.com/2022/day/5
#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Vec<Instruction>, Stacks) {
    let mut input = input.split("\n\n");
    let stacks = input.next().unwrap();
    let instrs = input
        .next()
        .unwrap()
        .lines()
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect_vec();
    let stacks = stacks.parse().unwrap();
    // println!("{:?}", instrs);
    (instrs, stacks)
}

#[aoc(day5, part1)]
pub fn solve_part1((instrs, stacks): &(Vec<Instruction>, Stacks)) -> String {
    instrs.into_iter().fold(stacks.clone(), |sum, next| sum.apply_part_one(next)).get_top()
}

#[aoc(day5, part2)]
pub fn solve_part2((instrs, stacks): &(Vec<Instruction>, Stacks)) -> String {
    instrs.into_iter().fold(stacks.clone(), |sum, next| sum.apply_part_two(next)).get_top()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("CMZ", solve_part1(&generator(provided)));
        assert_eq!(
            "CWMTGHBDW",
            solve_part1(&generator(include_str!("../input/2022/day5.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("MCD", solve_part2(&generator(provided)));
        assert_eq!(
            "SSCGWJCRB",
            solve_part2(&generator(include_str!("../input/2022/day5.txt")))
        );
    }
}
