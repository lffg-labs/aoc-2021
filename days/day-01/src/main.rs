use itertools::Itertools;

fn one(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn two(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    println!("one = {}", one(INPUT));
    println!("two = {}", two(INPUT));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_one() {
        assert_eq!(super::one(TEST_INPUT), 7);
    }

    #[test]
    fn test_two() {
        assert_eq!(super::two(TEST_INPUT), 5);
    }
}
