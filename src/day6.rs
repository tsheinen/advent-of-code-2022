use crate::prelude::*;
use std::str::FromStr;

/// https://adventofcode.com/2022/day/6
#[aoc_generator(day6)]
pub fn generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let (idx, _) = input.as_bytes().array_windows::<4>().enumerate().find(|(idx, chunk)| {
        chunk.iter().unique().count() == 4
    }).unwrap();
    idx + 4
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let (idx, _) = input.as_bytes().array_windows::<14>().enumerate().find(|(idx, chunk)| {
        chunk.iter().unique().count() == 14
    }).unwrap();
    idx + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "";
        assert_eq!(7, solve_part1(&generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
        assert_eq!(5, solve_part1(&generator("bvwbjplbgvbhsrlpgdmjqwftvncz")));
        assert_eq!(6, solve_part1(&generator("nppdvjthqldpwncqszvftbrmjlhg")));
        assert_eq!(10, solve_part1(&generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
        assert_eq!(11, solve_part1(&generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
        assert_eq!(
            1198,
            solve_part1(&generator(include_str!("../input/2022/day6.txt")))
        );
    }

        #[test]
        fn it_works_part2() {
            assert_eq!(19, solve_part2(&generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
            assert_eq!(23, solve_part2(&generator("bvwbjplbgvbhsrlpgdmjqwftvncz")));
            assert_eq!(23, solve_part2(&generator("nppdvjthqldpwncqszvftbrmjlhg")));
            assert_eq!(29, solve_part2(&generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
            assert_eq!(26, solve_part2(&generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
            assert_eq!(
                3120,
                solve_part2(&generator(include_str!("../input/2022/day6.txt")))
            );
        }
}
