fn one(input: &str) -> u64 {
    let mut horizontal = 0;
    let mut depth = 0;

    use Command::*;
    input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .for_each(|command| match command {
            Forward(val) => horizontal += val,
            Down(val) => depth += val,
            Up(val) => depth -= val,
        });

    horizontal * depth
}

fn two(input: &str) -> u64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    use Command::*;
    input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .for_each(|command| match command {
            Forward(val) => {
                horizontal += val;
                depth += aim * val;
            }
            Down(val) => aim += val,
            Up(val) => aim -= val,
        });

    horizontal * depth
}

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (name, val) = raw.trim().split_once(' ').ok_or(())?;
        let val: u64 = val.parse().map_err(|_| ())?;

        use Command::*;
        match name {
            "forward" => Ok(Forward(val)),
            "down" => Ok(Down(val)),
            "up" => Ok(Up(val)),
            _ => Err(()),
        }
    }
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
        assert_eq!(super::one(TEST_INPUT), 150);
    }

    #[test]
    fn test_two() {
        assert_eq!(super::two(TEST_INPUT), 900);
    }
}
