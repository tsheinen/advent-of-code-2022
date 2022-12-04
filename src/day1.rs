use crate::prelude::*;

/// https://adventofcode.com/2022/day/1
#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|x| x.split("\n").flat_map(|x| x.parse::<usize>()).collect_vec())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    input.into_iter().map(|x| x.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    input
        .into_iter()
        .map(|x| x.iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(24000, solve_part1(&generator(provided)));
        assert_eq!(
            69795,
            solve_part1(&generator(include_str!("../input/2022/day1.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(45000, solve_part2(&generator(provided)));
        assert_eq!(
            208437,
            solve_part2(&generator(include_str!("../input/2022/day1.txt")))
        );
    }
}
