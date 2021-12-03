#[derive(Debug)]
struct Entry {
    zero: u32,
    one: u32,
}

impl From<char> for Entry {
    fn from(char: char) -> Self {
        match char {
            '0' => Entry { zero: 1, one: 0 },
            '1' => Entry { zero: 0, one: 1 },
            _ => panic!("Invalid character."),
        }
    }
}

impl std::ops::AddAssign for Entry {
    fn add_assign(&mut self, rhs: Self) {
        self.zero += rhs.zero;
        self.one += rhs.one;
    }
}

fn sln(input: &str) -> String {
    let (gamma, epsilon) = input
        .lines()
        .fold(Vec::<Entry>::new(), |mut acc, curr| {
            for (i, char) in curr.chars().enumerate() {
                match acc.get_mut(i) {
                    Some(entry) => *entry += char.into(),
                    None => acc.insert(i, char.into()),
                }
            }
            acc
        })
        .iter()
        .rev()
        .enumerate()
        .fold((0, 0), |(gamma, epsilon), (i, entry)| {
            use std::cmp::Ordering::*;
            match entry.one.cmp(&entry.zero) {
                Greater => (gamma + 2u32.pow(i as _), epsilon),
                Less => (gamma, epsilon + 2u32.pow(i as _)),
                Equal => panic!("Unknown state."),
            }
        });
    (gamma * epsilon).to_string()
}

fn main() {
    let out = sln(include_str!("../input.txt"));
    println!("{}", out);
}

#[cfg(test)]
#[test]
fn test() {
    let out = sln(include_str!("../test.txt"));
    assert_eq!(out, "198");
}
