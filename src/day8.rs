use crate::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
    length: usize,
}

impl Position {
    fn to_index(&self) -> usize {
        assert!(self.x < self.length);
        assert!(self.y < self.length);
        self.y * self.length + self.x
    }

    fn from_index(length: usize, index: usize) -> Self {
        Position {
            x: index % length,
            y: index / length,
            length: length,
        }
    }

    fn shift(&self, dir: Direction) -> Option<Position> {
        let mut out = self.clone();
        if (out.y == 0 && dir == Direction::North)
            || (out.y == 0 && dir == Direction::South)
            || (out.x == 0 && dir == Direction::East)
            || (out.x == 0 && dir == Direction::West)
        {
            return None;
        }
        match dir {
            Direction::North => out.y += 1,
            Direction::South => out.y -= 1,
            Direction::East => out.x += 1,
            Direction::West => out.x -= 1,
        }
        if out.y >= self.length || out.x >= self.length {
            None
        } else {
            Some(out)
        }
    }

    fn line_of_sight(&self, dir: Direction) -> impl Iterator<Item = Position> + '_ {
        std::iter::successors(Some(self.clone()), move |n| n.shift(dir)).skip(1)
    }

    fn all_visible(&self) -> impl Iterator<Item = Position> + '_ {
        self.line_of_sight(Direction::North)
            .chain(self.line_of_sight(Direction::South))
            .chain(self.line_of_sight(Direction::East))
            .chain(self.line_of_sight(Direction::West))
    }
}

pub struct Forest {
    length: usize,
    heights: Vec<usize>,
}

impl Forest {
    fn get_height(&self, pos: &Position) -> usize {
        self.heights[pos.to_index()]
    }

    fn is_visible(&self, pos: &Position) -> bool {
        if pos.x == 0 || pos.y == 0 || pos.x == self.length - 1 || pos.y == self.length - 1 {
            return true;
        }
        let height = self.get_height(pos);
        pos.line_of_sight(Direction::North)
            .map(|x| self.get_height(&x))
            .all(|x| x < height)
            || pos
                .line_of_sight(Direction::South)
                .map(|x| self.get_height(&x))
                .all(|x| x < height)
            || pos
                .line_of_sight(Direction::East)
                .map(|x| self.get_height(&x))
                .all(|x| x < height)
            || pos
                .line_of_sight(Direction::West)
                .map(|x| self.get_height(&x))
                .all(|x| x < height)
    }

    fn all(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.heights.len()).map(|index| Position::from_index(self.length, index))
    }

    fn viewing_distance(&self, pos: &Position, dir: Direction) -> usize {
        let height = self.get_height(pos);
        
        let mut r = 0;

        // tfw no take_while_inclusive
        for x in pos.line_of_sight(dir).map(|x| self.get_height(&x)) {
            r += 1;
            if x >= height {
                break
            }
        }
        r
    }

    fn scenic_score(&self, pos: &Position) -> usize {
        self.viewing_distance(pos, Direction::North)
            * self.viewing_distance(pos, Direction::South)
            * self.viewing_distance(pos, Direction::East)
            * self.viewing_distance(pos, Direction::West)
    }
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.lines().count();
        assert_eq!(s.lines().count(), s.lines().next().unwrap().len());

        let heights = s
            .lines()
            .flat_map(|line| line.chars().map(|ch| ch as usize - 0x30))
            .collect();

        Ok(Forest {
            length: length,
            heights: heights,
        })
    }
}

/// https://adventofcode.com/2022/day/8
#[aoc_generator(day8)]
pub fn generator(input: &str) -> Forest {
    input.parse().unwrap()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Forest) -> usize {
    input.all().filter(|x| input.is_visible(x)).count()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Forest) -> usize {
    input.all().map(|pos| input.scenic_score(&pos)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "30373
25512
65332
33549
35390";
        assert_eq!(21, solve_part1(&generator(provided)));
        assert_eq!(
            1820,
            solve_part1(&generator(include_str!("../input/2022/day8.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "30373
25512
65332
33549
35390";
        assert_eq!(8, solve_part2(&generator(provided)));
        assert_eq!(
            385112,
            solve_part2(&generator(include_str!("../input/2022/day8.txt")))
        );
    }
}
