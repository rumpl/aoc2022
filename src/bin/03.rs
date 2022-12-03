use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);

                let a: HashSet<char> = first.chars().collect();
                let b: HashSet<char> = second.chars().collect();

                let n = a.intersection(&b).collect::<Vec<&char>>();

                match *n[0] {
                    'a'..='z' => *n[0] as u32 - 97 + 1,
                    'A'..='Z' => *n[0] as u32 - 65 + 27,
                    _ => panic!("ugh"),
                }
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|a| {
                let f: HashSet<char> = a[0].chars().collect();
                let g: HashSet<char> = a[1].chars().collect();
                let h: HashSet<char> = a[2].chars().collect();

                let j: HashSet<char> = f.intersection(&g).copied().collect();
                let n = j.intersection(&h).collect::<Vec<&char>>();
                match *n[0] {
                    'a'..='z' => *n[0] as u32 - 97 + 1,
                    'A'..='Z' => *n[0] as u32 - 65 + 27,
                    _ => panic!("ugh"),
                }
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
