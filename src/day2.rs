#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(&self) -> i64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play(&self, sh: &Shape) -> i64 {
        let win = match (self, sh) {
            (Shape::Rock, Shape::Scissors) => true,
            (Shape::Scissors, Shape::Paper) => true,
            (Shape::Paper, Shape::Rock) => true,
            _ => false,
        };
        let outcome_points = if win {
            6
        } else if self == sh {
            3
        } else {
            0
        };
        outcome_points + self.points()
    }

    fn play_round2(&self, theirs: &Shape) -> i64 {
        let play = match self {
            // we match shapes to outcomes for round 2 bc i am shit lol
            Shape::Rock => {
                // trying to lose
                match theirs {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            }
            Shape::Paper => *theirs, // trying to draw
            Shape::Scissors => {
                match theirs {
                    // trying to win,
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            }
        };

        let win = match (play, theirs) {
            (Shape::Rock, Shape::Scissors) => true,
            (Shape::Scissors, Shape::Paper) => true,
            (Shape::Paper, Shape::Rock) => true,
            _ => false,
        };
        let outcome_points = if win {
            6
        } else if play == *theirs {
            3
        } else {
            0
        };
        outcome_points + play.points()
    }

    fn new(chr: &str) -> Self {
        match chr {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("invalid shape {}", chr),
        }
    }
}

/// https://adventofcode.com/2022/day/2
#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(Shape, Shape)> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut sp = x.split(" ");
            (
                Shape::new(sp.next().unwrap()),
                Shape::new(sp.next().unwrap()),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Shape, Shape)]) -> i64 {
    input.into_iter().map(|(other, own)| own.play(other)).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Shape, Shape)]) -> i64 {
    input
        .into_iter()
        .map(|(theirs, own)| own.play_round2(theirs))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "A Y
B X
C Z";
        assert_eq!(15, solve_part1(&generator(provided)));
        assert_eq!(
            14069,
            solve_part1(&generator(include_str!("../input/2022/day2.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "A Y
B X
C Z";
        assert_eq!(12, solve_part2(&generator(provided)));
        assert_eq!(
            12411,
            solve_part2(&generator(include_str!("../input/2022/day2.txt")))
        );
    }
}
