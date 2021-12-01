fn sln(input: &str) -> String {
    let (count, _) = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .fold((0, i32::MAX), |(count, prev), curr| {
            (count + if curr > prev { 1 } else { 0 }, curr)
        });
    count.to_string()
}

fn main() {
    let out = sln(include_str!("../input.txt"));
    println!("{}", out);
}

#[cfg(test)]
#[test]
fn test() {
    let out = sln(include_str!("../test.txt"));
    assert_eq!(out, "7");
}
