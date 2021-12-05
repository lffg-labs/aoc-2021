use std::{borrow::BorrowMut, fmt::Debug, iter::Sum, str::FromStr};

const BOARD_SIZE: usize = 5;

fn one(input: &str) -> u32 {
    let (numbers, mut boards) = parse_numbers_and_boards::<u32>(input);
    for number in numbers {
        for board in boards.iter_mut() {
            if let Some((_, true)) = board.try_update(number) {
                return board.unmarked_sum() * number;
            }
        }
    }
    unreachable!("Invalid input.");
}

fn two(input: &str) -> u32 {
    let (numbers, mut boards) = parse_numbers_and_boards::<u32>(input);
    let mut last_win_product: Option<u32> = None;

    for number in numbers {
        for board in boards.iter_mut() {
            if board.won() {
                continue;
            }
            if let Some((_, true)) = board.try_update(number) {
                last_win_product = Some(board.unmarked_sum() * number);
            }
        }
    }

    last_win_product.unwrap()
}

/// Returns a tuple with the input numbers and boards.
fn parse_numbers_and_boards<'s, T>(
    input: &'s str,
) -> (impl Iterator<Item = T> + 's, Vec<BingoBoard<T>>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<T>().unwrap());

    let mut boards: Vec<BingoBoard<T>> = Vec::new();
    loop {
        // Each board is preceded by a blank new line.
        if let None = lines.next() {
            break;
        }
        let board_iter = lines
            .borrow_mut()
            .take(BOARD_SIZE)
            .map(|s| s.split(' '))
            .flatten()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap());
        boards.push(BingoBoard::from_iter(BOARD_SIZE, board_iter));
    }

    (numbers, boards)
}

#[derive(Debug, Copy, Clone)]
struct BingoCoord {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BingoMarker {
    Marked,
    Unmarked,
}

#[derive(Debug)]
struct BingoSlot<T> {
    kind: BingoMarker,
    data: T,
}

#[derive(Debug, Default)]
struct BingoBoard<T> {
    size: usize,
    won: bool,
    rows: Vec<Vec<BingoSlot<T>>>,
}

impl<T> BingoBoard<T> {
    /// Creates a new board from the given iterator.
    ///
    /// # Panics
    ///
    /// Panics if the iterator cannot yield at least `size * size` items.
    pub fn from_iter(size: usize, mut iter: impl Iterator<Item = T>) -> Self {
        let mut rows = Vec::with_capacity(size);
        for _ in 0..size {
            let mut row = Vec::with_capacity(size);
            row.extend((&mut iter).take(size).map(|data| BingoSlot {
                kind: BingoMarker::Unmarked,
                data,
            }));
            assert_eq!(row.len(), size, "Must yield size * size elements.");
            rows.push(row);
        }
        BingoBoard {
            size,
            rows,
            won: false,
        }
    }

    /// Returns a boolean indicating the winning status.
    #[inline]
    pub fn won(&self) -> bool {
        return self.won;
    }

    /// Returns an iterator that yields references to all slots in the board.
    pub fn all(&self) -> impl Iterator<Item = &BingoSlot<T>> {
        self.rows.iter().map(|row| row.iter()).flatten()
    }

    /// Returns an iterator that yields references to the elements in the given column index.
    pub fn col(&self, index: usize) -> impl Iterator<Item = &BingoSlot<T>> {
        assert!(index < self.size, "Index out of bounds.");
        self.rows.iter().map(move |r| r.iter().nth(index).unwrap())
    }

    /// Returns an iterator that yields references to the elements in the given row index.
    pub fn row(&self, index: usize) -> impl Iterator<Item = &BingoSlot<T>> {
        assert!(index < self.size, "Index out of bounds.");
        self.rows[index].iter()
    }

    /// Tries to update the board with the given data.
    ///
    /// If the update is possible, a tuple with two elements is returned:
    /// 1. The insertion coordinate, [`BoardCoord`].
    /// 2. A boolean that indicates if the update is victorious.
    ///
    /// If the update is not possible, None is returned.
    pub fn try_update(&mut self, data: T) -> Option<(BingoCoord, bool)>
    where
        T: PartialEq,
    {
        for (i, row) in self.rows.iter_mut().enumerate() {
            for (j, slot) in row.iter_mut().enumerate() {
                if slot.kind == BingoMarker::Unmarked && slot.data == data {
                    slot.kind = BingoMarker::Marked;
                    let coord = BingoCoord { row: i, col: j };
                    if self.is_winning_coord(coord) {
                        self.won = true;
                    }
                    return Some((coord, self.won));
                }
            }
        }
        None
    }

    /// Checks if the given column is in a winning state.
    pub fn is_winning_col(&self, index: usize) -> bool {
        self.col(index).all(|slot| slot.kind == BingoMarker::Marked)
    }

    /// Checks if the given row is in a winning state.
    pub fn is_winning_row(&self, index: usize) -> bool {
        self.row(index).all(|slot| slot.kind == BingoMarker::Marked)
    }

    /// Checks if the row or column of the given [`BingoCoord`] are in a winning state.
    pub fn is_winning_coord(&self, coord: BingoCoord) -> bool {
        self.is_winning_col(coord.col) || self.is_winning_row(coord.row)
    }

    /// Returns the sum of the unmarked slots.
    pub fn unmarked_sum(&self) -> T
    where
        T: Sum<T> + Clone,
    {
        self.all()
            .filter(|slot| slot.kind == BingoMarker::Unmarked)
            .map(|slot| slot.data.clone())
            .sum()
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
        assert_eq!(super::one(TEST_INPUT), 4512);
    }

    #[test]
    fn test_two() {
        assert_eq!(super::two(TEST_INPUT), 1924);
    }
}
