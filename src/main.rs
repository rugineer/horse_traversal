use std::fmt;
use std::fmt::Formatter;

const SIZE: usize = 8;
const LETTERS: [char; SIZE] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
struct Board([[u8; SIZE]; SIZE]);

struct BoardNotation {
    letter: char,
    digit: usize,
}

impl From<(usize, usize)> for BoardNotation {
    fn from(value: (usize, usize)) -> Self {
        let letter = {
            if value.0 >= SIZE {
                LETTERS[SIZE - 1]
            } else {
                LETTERS[value.0]
            }
        };
        let digit = {
            if value.1 >= SIZE {
                SIZE
            } else {
                value.1 + 1
            }
        };
        BoardNotation { letter, digit }
    }
}

impl From<BoardNotation> for (usize, usize) {
    fn from(value: BoardNotation) -> Self {
        let col = {
            let letter = value.letter.to_ascii_uppercase();
            let col = match LETTERS.iter().position(|&c| c == letter) {
                Some(c) => c,
                None => SIZE - 1,
            };
            col
        };
        let row = {
            if value.digit >= SIZE {
                SIZE - 1
            } else {
                value.digit
            }
        };
        (col, row)
    }
}

impl fmt::Display for BoardNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.letter, self.digit)
    }
}

impl Board {
    fn new() -> Self {
        Self([[0; SIZE]; SIZE])
    }
    fn get(&self, col: usize, row: usize) -> Option<u8> {
        if col > (SIZE - 1) || row > (SIZE - 1) {
            return None;
        }
        Some(self.0[row][col])
    }
    fn set(&mut self, col: usize, row: usize, value: u8) -> Result<(), String> {
        if col > (SIZE - 1) || row > (SIZE - 1) {
            return Err(String::from("Индекс вне диапазона"));
        }
        self.0[row][col] = value;
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  ")?;
        for letter in LETTERS {
            write!(f, "{:>3}", letter)?;
        }
        writeln!(f, "\n    {}", "-".repeat(3 * SIZE - 2))?;
        for i in 1..=self.0.len() {
            write!(f, "{}|", SIZE - i)?;
            for cell in self.0[SIZE - i] {
                write!(f, "{:>3}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// fn next(col: usize, row: usize, field: &Board) -> Option<(usize, usize)> {}

fn main() -> Result<(), String> {
    let mut board = Board::new();
    board.set(0, 7, 1)?;
    println!("{}", board);
    board.set(3, 3, 2)?;
    println!("{}", board);
    println!("\n{}", board.get(3, 3).unwrap());
    println!("{}", BoardNotation::from((3, 0)));
    Ok(())
}
