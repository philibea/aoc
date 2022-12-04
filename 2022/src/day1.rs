
#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    return lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max()
        .unwrap();

}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let mut groups = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<_>>();

    groups.sort_by_key(|&v| u64::MAX - v);

    let max = &groups[0..3].iter().sum::<u64>();

    return *max;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        const SAMPLE_1: &str = "1\n2\n3\n\n4\n5\n\n6\n\n7\n8\n";

        assert_eq!(part1(SAMPLE_1), 15);
    }

    #[test]
    fn sample2() {
        const SAMPLE_1: &str = "1\n2\n3\n\n4\n5\n\n6\n\n7\n8\n";
        assert_eq!(part2(SAMPLE_1), 30);
    }
}
