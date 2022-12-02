enum Choice {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn rps(input: &str) -> Choice {
    match input {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissor,
        _ => panic!("ugh"),
    }
}

fn rps2(input: &str) -> Choice {
    match input {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissor,
        _ => panic!("ugh"),
    }
}

fn outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("ugh"),
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|a| a.split(' '))
            .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
            .map(|(a, b)| (rps(a), rps2(b)))
            .fold(0, |acc, (x, y)| {
                let aa = match (x, y) {
                    (Choice::Rock, Choice::Rock) => 3 + 1,
                    (Choice::Rock, Choice::Paper) => 6 + 2,
                    (Choice::Rock, Choice::Scissor) => 3,

                    (Choice::Paper, Choice::Rock) => 1,
                    (Choice::Paper, Choice::Paper) => 3 + 2,
                    (Choice::Paper, Choice::Scissor) => 6 + 3,

                    (Choice::Scissor, Choice::Rock) => 6 + 1,
                    (Choice::Scissor, Choice::Paper) => 2,
                    (Choice::Scissor, Choice::Scissor) => 3 + 3,
                };

                aa + acc
            }),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|a| a.split(' '))
            .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
            .map(|(a, b)| (rps(a), outcome(b)))
            .fold(0, |acc, (x, y)| {
                let aa = match (x, y) {
                    (Choice::Rock, Outcome::Lose) => 3,
                    (Choice::Rock, Outcome::Draw) => 3 + 1,
                    (Choice::Rock, Outcome::Win) => 6 + 2,

                    (Choice::Paper, Outcome::Lose) => 1,
                    (Choice::Paper, Outcome::Draw) => 3 + 2,
                    (Choice::Paper, Outcome::Win) => 6 + 3,

                    (Choice::Scissor, Outcome::Lose) => 2,
                    (Choice::Scissor, Outcome::Draw) => 3 + 3,
                    (Choice::Scissor, Outcome::Win) => 6 + 1,
                };

                aa + acc
            }),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
