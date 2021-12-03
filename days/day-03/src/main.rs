use std::{
    cmp::Ordering,
    ops::{AddAssign, Neg},
};

fn one(input: &str) -> u32 {
    let stats = compute_stats(input.lines());

    let mut gamma = 0;
    let mut episilon = 0;

    for (i, stat) in stats.iter().enumerate() {
        use Ordering::*;
        match stat.one.cmp(&stat.zero) {
            Greater => gamma += 2u32.pow(i as _),
            Less => episilon += 2u32.pow(i as _),
            Equal => panic!("Invalid state."),
        }
    }

    gamma * episilon
}

fn two(input: &str) -> u32 {
    let o2_rating = compute_gas_rating(input, StatMode::MostCommon);
    let co2_rating = compute_gas_rating(input, StatMode::LeastCommon);

    o2_rating * co2_rating
}

fn compute_stats<'s>(source: impl Iterator<Item = &'s str>) -> Vec<Stat> {
    let mut lines = source.peekable();
    let col_count = lines.peek().unwrap().len();

    lines
        // Yields from least significant to most significant bit.
        //                       ↓↓↓
        .map(|line| line.chars().rev().map(Bit::from))
        .fold(vec![Stat::default(); col_count], |mut vec, curr_bits| {
            for (i, bit) in curr_bits.enumerate() {
                vec[i] += bit;
            }
            vec
        })
}

fn compute_gas_rating(input: &str, mode: StatMode) -> u32 {
    let mut vec: Vec<_> = input.lines().collect();
    let mut curr = 0;

    while vec.len() > 1 {
        let stats = compute_stats(vec.iter().map(|r| *r));
        vec = vec
            .into_iter()
            .filter(|line| {
                let expected =
                    stats[stats.len() - 1 - curr]
                        .query_by_mode(mode)
                        .unwrap_or(match mode {
                            StatMode::MostCommon => Bit::One,
                            StatMode::LeastCommon => Bit::Zero,
                        });
                expected == line.chars().nth(curr).unwrap().into()
            })
            .collect();
        curr += 1;
    }

    u32::from_str_radix(vec[0], 2).unwrap()
}

#[derive(Debug, Copy, Clone)]
enum StatMode {
    MostCommon,
    LeastCommon,
}

#[derive(Debug, Copy, Clone, Default)]
struct Stat {
    zero: u32,
    one: u32,
}

impl Stat {
    /// Returns the most or least common [`Bit`] according to the given
    /// [`StatMode`].
    ///
    /// Returns an Option. The None variant is returned if the frequency of both
    /// [`Bit`]s are equal.
    pub fn query_by_mode(&self, mode: StatMode) -> Option<Bit> {
        use Ordering::*;
        use StatMode::*;
        match mode {
            MostCommon => match self.one.cmp(&self.zero) {
                Greater => Some(Bit::One),
                Less => Some(Bit::Zero),
                Equal => None,
            },
            LeastCommon => self.query_by_mode(MostCommon).map(Neg::neg),
        }
    }
}

impl AddAssign<Bit> for Stat {
    fn add_assign(&mut self, rhs: Bit) {
        match rhs {
            Bit::Zero => self.zero += 1,
            Bit::One => self.one += 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bit {
    Zero,
    One,
}

impl Neg for Bit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Bit::*;
        match self {
            Zero => One,
            One => Zero,
        }
    }
}

impl From<char> for Bit {
    fn from(char: char) -> Self {
        match char {
            '0' => Bit::Zero,
            '1' => Bit::One,
            _ => panic!("Invalid attempt to convert char to Bit."),
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
        assert_eq!(super::one(TEST_INPUT), 198);
    }

    #[test]
    fn test_two() {
        assert_eq!(super::two(TEST_INPUT), 230);
    }
}
