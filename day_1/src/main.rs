use itertools::Itertools;

#[derive(Clone)]
struct Entry {
    left: u32,
    right: u32,
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u32 {
    let mut entries_left_sorted = parse_entries(input);

    entries_left_sorted.sort_by_key(|e| e.left);

    let mut entries_right_sorted = entries_left_sorted.clone();
    entries_right_sorted.sort_by_key(|e| e.right);

    let mut sum = 0;
    let mut right_itr = entries_right_sorted.iter();
    for left in entries_left_sorted.iter() {
        let Some(right) = right_itr.next() else {
            return 0;
        };

        sum += left.left.abs_diff(right.right);
    }

    sum
}

fn solve_part_2(input: &str) -> u32 {
    let entries = parse_entries(input);
    let right_counts = entries.iter().map(|e| e.right).counts();

    entries
        .iter()
        .map(|e| e.left * *right_counts.get(&e.left).unwrap_or(&0) as u32)
        .sum()
}

fn parse_entries(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("   ");
            let left = split.next().unwrap().parse().unwrap();
            let right = split.next().unwrap().parse().unwrap();

            Entry { left, right }
        })
        .collect()
}

#[cfg(test)]
mod day_1_tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve_part_1(input), 11);
    }

    #[test]
    fn part_1() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_part_1(input), 2430334);
    }

    #[test]
    fn part_2_example() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve_part_2(input), 31);
    }

    #[test]
    fn part_2() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_part_2(input), 28786472);
    }
}
