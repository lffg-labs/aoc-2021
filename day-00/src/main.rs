fn one(_input: &str) -> usize {
    0
}

fn two(_input: &str) -> usize {
    0
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
        assert_eq!(super::one(TEST_INPUT), 0);
    }

    #[test]
    fn test_two() {
        assert_eq!(super::two(TEST_INPUT), 0);
    }
}
