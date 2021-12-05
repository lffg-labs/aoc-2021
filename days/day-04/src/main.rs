use std::borrow::BorrowMut;

const BOARD_SIZE: usize = 5;

fn one(input: &str) -> u32 {
    let mut lines = input.trim().lines();
    let numbers = parse_numbers(lines.next().unwrap());

    let mut boards: Vec<BingoBoard<u32>> = Vec::new();
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

    for number in numbers {
        for board in boards.iter_mut() {
            if let Some(coord) = board.try_update(number) {
                if board.is_won_coord(coord) {
                    let unmarked_sum: u32 = board
                        .all()
                        .filter(|slot| slot.kind == BingoMarker::Unmarked)
                        .map(|slot| slot.data)
                        .sum();
                    return unmarked_sum * number;
                }
            }
        }
    }

    unreachable!("Invalid input.");
}

fn parse_numbers<'s>(raw_seq: &'s str) -> impl Iterator<Item = u32> + 's {
    raw_seq.split(',').map(|s| s.parse().unwrap())
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
        BingoBoard { size, rows }
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
    /// Returns some [`BingoCoord`] if the supplied data exists in the board and is unmarked,
    /// otherwise `None` is returned.
    pub fn try_update(&mut self, data: T) -> Option<BingoCoord>
    where
        T: PartialEq,
    {
        for (i, row) in self.rows.iter_mut().enumerate() {
            for (j, slot) in row.iter_mut().enumerate() {
                if slot.kind == BingoMarker::Unmarked && slot.data == data {
                    slot.kind = BingoMarker::Marked;
                    return Some(BingoCoord { row: i, col: j });
                }
            }
        }
        None
    }

    /// Checks if the given column is in a winning state.
    pub fn is_won_col(&self, index: usize) -> bool {
        self.col(index).all(|slot| slot.kind == BingoMarker::Marked)
    }

    /// Checks if the given row is in a winning state.
    pub fn is_won_row(&self, index: usize) -> bool {
        self.row(index).all(|slot| slot.kind == BingoMarker::Marked)
    }

    /// Checks if the row or column of the given [`BingoCoord`] are in a winning state.
    pub fn is_won_coord(&self, coord: BingoCoord) -> bool {
        self.is_won_col(coord.col) || self.is_won_row(coord.row)
    }
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
        assert_eq!(super::one(TEST_INPUT), 4512);
    }

    #[test]
    fn test_two() {
        assert_eq!(super::two(TEST_INPUT), 0);
    }
}
