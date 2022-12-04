use crate::prelude::*;

#[derive(Debug)]
pub struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl Rucksack {
    fn find_repeated(&self) -> char {
        let repeated = self.left.intersection(&self.right).collect::<Vec<_>>();
        assert_eq!(repeated.len(), 1);
        *repeated[0]
    }

    fn union(&self) -> HashSet<char> {
        self.left.union(&self.right).copied().collect()
    }
}

fn priority(chr: char) -> i64 {
    match chr {
        'a'..='z' => chr as i64 - 0x60,
        'A'..='Z' => chr as i64 - 0x40 + 26,
        _ => panic!("invalid char"),
    }
}

fn intersection<const N: usize>(rucksacks: &[Rucksack; N]) -> char {
    assert!(rucksacks.len() > 0);
    let shared = rucksacks
        .into_iter()
        .map(Rucksack::union)
        .reduce(|accum, next| &accum & &next)
        .unwrap();
    assert_eq!(shared.len(), 1);
    *shared.iter().next().unwrap()
}

/// https://adventofcode.com/2022/day/3
#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|rucksack| {
            assert_eq!(rucksack.len() % 2, 0);
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            Rucksack {
                left: left.chars().collect(),
                right: right.chars().collect(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> i64 {
    input
        .into_iter()
        .map(Rucksack::find_repeated)
        .map(priority)
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Rucksack]) -> i64 {
    input
        .array_chunks::<3>()
        .map(intersection)
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, solve_part1(&generator(provided)));
        assert_eq!(
            8085,
            solve_part1(&generator(include_str!("../input/2022/day3.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(70, solve_part2(&generator(provided)));
        assert_eq!(
            2515,
            solve_part2(&generator(include_str!("../input/2022/day3.txt")))
        );
    }
}
