fn elves(input: &str) -> Vec<&str> {
    input.split("\n\n").collect::<Vec<&str>>()
}

fn elve_calories(input: &str) -> u32 {
    input
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|n| n.parse::<u32>().unwrap())
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    elves(input).into_iter().map(elve_calories).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n = elves(input)
        .into_iter()
        .map(elve_calories)
        .collect::<Vec<u32>>();

    n.sort_by(|a, b| b.cmp(a));

    Some(n.into_iter().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
