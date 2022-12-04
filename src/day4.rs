use crate::prelude::*;

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{lo}-{hi}")]
pub struct SectionRange {
    lo: i64,
    hi: i64,
}

impl SectionRange {
    fn is_fully_within(&self, other: &SectionRange) -> bool {
        other.lo >= self.lo && other.hi <= self.hi
    }

    fn is_partially_within(&self, other: &SectionRange) -> bool {
        other.lo >= self.lo && other.lo <= self.hi || other.hi >= self.lo && other.hi <= self.hi
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone)]
#[display("{lhs},{rhs}")]
pub struct Pair {
    pub lhs: SectionRange,
    pub rhs: SectionRange,
}

impl Pair {
    fn is_fully_contained(&self) -> bool {
        self.lhs.is_fully_within(&self.rhs) || self.rhs.is_fully_within(&self.lhs)
    }

    fn is_partially_contained(&self) -> bool {
        self.lhs.is_partially_within(&self.rhs) || self.rhs.is_partially_within(&self.lhs)
    }
}

/// https://adventofcode.com/2022/day/4
#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Pair> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pair]) -> usize {
    input
        .into_iter()
        .cloned()
        .filter(Pair::is_fully_contained)
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pair]) -> usize {
    input
        .into_iter()
        .cloned()
        .filter(Pair::is_partially_contained)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(2, solve_part1(&generator(provided)));
        assert_eq!(
            475,
            solve_part1(&generator(include_str!("../input/2022/day3.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(4, solve_part2(&generator(provided)));
        assert_eq!(
            825,
            solve_part2(&generator(include_str!("../input/2022/day3.txt")))
        );
    }
}
