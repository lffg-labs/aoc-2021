// I don't like this. TODO: Refactor?
fn windows_n<T: Clone>(n: usize, iter: impl Iterator<Item = T>) -> Vec<Vec<T>> {
    iter.fold(vec![vec![]], |mut vec, curr| {
        let last = vec.last_mut().unwrap();
        if last.len() < n {
            last.push(curr);
        } else {
            let mut new = Vec::from(&last[1..]);
            new.push(curr);
            vec.push(new);
        }
        vec
    })
}

fn sln(input: &str) -> String {
    let iter = input.lines().map(|line| line.parse::<i32>().unwrap());
    let (count, _) = windows_n(3, iter)
        .iter()
        .map(|window| window.iter().sum())
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
    assert_eq!(out, "5");
}
