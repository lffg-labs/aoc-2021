use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(raw_command: &str) -> Result<Self, Self::Err> {
        let (name, val) = raw_command.split_once(' ').ok_or(())?;
        let val: u32 = val.parse().map_err(|_| ())?;

        Ok(match name {
            "forward" => Command::Forward(val),
            "down" => Command::Down(val),
            "up" => Command::Up(val),
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Default)]
struct Pos {
    horizontal: u32,
    depth: u32,
}

impl Pos {
    pub fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Forward(val) => self.horizontal += val,
            Command::Down(val) => self.depth += val,
            Command::Up(val) => self.depth -= val,
        }
    }

    pub fn product(&self) -> u32 {
        self.horizontal * self.depth
    }
}

fn sln(input: &str) -> String {
    let mut pos = Pos::default();

    let res: Result<(), ()> = input
        .lines()
        .map(Command::from_str)
        .try_for_each(|command| {
            pos.apply_command(&command?);
            Ok(())
        });
    assert!(res.is_ok());

    pos.product().to_string()
}

fn main() {
    let out = sln(include_str!("../input.txt"));
    println!("{}", out);
}

#[cfg(test)]
#[test]
fn test() {
    let out = sln(include_str!("../test.txt"));
    assert_eq!(out, "150");
}
