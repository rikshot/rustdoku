use itertools::Itertools;
use std::{error::Error, fmt, str::FromStr};
use thiserror::Error;

use super::candidates::Candidates;

#[derive(Clone, Copy, Debug)]
struct Cell {
    pub value: u8,
    pub candidates: Candidates,
    pub frozen: bool,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Cell {
            value,
            candidates: Candidates::new(value == 0),
            frozen: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Grid {
    cells: [Cell; 81],
}

const fn insert<const N: usize>(mut array: [usize; N], value: usize) -> [usize; N] {
    let mut index = 0;
    while index < array.len() {
        if array[index] == 0 {
            array[index] = value;
            break;
        }
        index += 1;
    }
    array
}

type SudokuIndices = [[usize; 9]; 9];
type PeerIndices = [[usize; 20]; 81];

const fn indices() -> (SudokuIndices, SudokuIndices, SudokuIndices, PeerIndices) {
    let mut rows = [[0; 9]; 9];
    let mut columns = [[0; 9]; 9];
    let mut boxes = [[0; 9]; 9];
    let mut peers = [[0; 20]; 81];

    let mut index = 0;
    while index < 81 {
        let row_index = index / 9;
        let column_index = index % 9;
        let box_index = row_index / 3 * 3 + column_index / 3;
        rows[row_index] = insert(rows[row_index], index);
        columns[column_index] = insert(columns[column_index], index);
        boxes[box_index] = insert(boxes[box_index], index);
        index += 1;
    }

    let mut index = 0;
    while index < 81 {
        let row_index = index / 9;
        let column_index = index % 9;
        let box_index = row_index / 3 * 3 + column_index / 3;
        let mut peer_index = 0;
        while peer_index < 9 {
            let peer = rows[row_index][peer_index];
            if peer != index {
                peers[index] = insert(peers[index], peer);
            }
            peer_index += 1;
        }
        let mut peer_index = 0;
        while peer_index < 9 {
            let peer = columns[column_index][peer_index];
            if peer != index {
                peers[index] = insert(peers[index], peer);
            }
            peer_index += 1;
        }
        let mut peer_index = 0;
        while peer_index < 9 {
            let peer = boxes[box_index][peer_index];
            let peer_row_index = peer / 9;
            let peer_column_index = peer % 9;
            if peer_row_index != row_index && peer_column_index != column_index {
                peers[index] = insert(peers[index], peer);
            }
            peer_index += 1;
        }
        index += 1;
    }

    (rows, columns, boxes, peers)
}

static INDICES: (SudokuIndices, SudokuIndices, SudokuIndices, PeerIndices) = indices();
pub static ROWS: &SudokuIndices = &INDICES.0;
pub static COLUMNS: &SudokuIndices = &INDICES.1;
pub static BOXES: &SudokuIndices = &INDICES.2;
pub static PEERS: &PeerIndices = &INDICES.3;

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: [Cell::new(0); 81],
        }
    }

    pub fn cells(&'_ self) -> impl Iterator<Item = u8> + '_ {
        self.cells.iter().map(|cell| cell.value)
    }

    pub fn get(&self, index: usize) -> u8 {
        debug_assert!(index < 81);
        self.cells[index].value
    }

    pub fn set(&mut self, index: usize, value: u8, checked: bool) -> bool {
        debug_assert!(index < 81);
        debug_assert!(value < 10);
        let mut cell = &mut self.cells[index];
        if !cell.frozen {
            cell.value = value;
            if checked {
                return self.update_candidates(index);
            }
            if value > 0 {
                cell.candidates.unset_all();
            } else {
                cell.candidates.set_all();
            }
            return true;
        }
        false
    }

    pub fn update_candidates(&mut self, index: usize) -> bool {
        debug_assert!(index < 81);
        let mut cells = self.cells;
        let value = cells[index].value;
        if value > 0 {
            let cell = Cell::new(value);
            for peer in PEERS[index] {
                let peer = &mut cells[peer];
                if peer.value == 0 {
                    peer.candidates.unset((cell.value - 1) as usize);
                    if peer.candidates.none() {
                        return false;
                    }
                } else if cell.value == peer.value {
                    return false;
                }
            }
            cells[index] = cell;
        } else {
            let old_value = cells[index].value;
            let mut cell = Cell::new(0);
            for peer in PEERS[index] {
                let peer = &mut cells[peer];
                if peer.value > 0 {
                    cell.candidates.unset(peer.value as usize - 1);
                    if cell.candidates.none() {
                        return false;
                    }
                } else if old_value > 0 {
                    peer.candidates.set(old_value as usize - 1);
                }
            }
            cells[index] = cell;
        }
        self.cells = cells;
        true
    }

    pub fn candidates(&self, index: usize) -> &Candidates {
        debug_assert!(index < 81);
        &self.cells[index].candidates
    }

    pub fn candidates_mut(&mut self, index: usize) -> &mut Candidates {
        debug_assert!(index < 81);
        &mut self.cells[index].candidates
    }

    pub fn freeze(&mut self, index: usize) {
        debug_assert!(index < 81);
        self.cells[index].frozen = true;
    }

    pub fn frozen(&self, index: usize) -> bool {
        debug_assert!(index < 81);
        self.cells[index].frozen
    }

    pub fn givens(&self) -> usize {
        self.cells.iter().filter(|cell| cell.frozen).count()
    }

    pub fn is_complete(&self) -> bool {
        !self.cells.iter().any(|cell| cell.value == 0)
    }

    pub fn is_valid(&self) -> bool {
        !self.cells.iter().enumerate().any(|(index, cell)| {
            cell.value > 0
                && PEERS[index]
                    .iter()
                    .map(|peer| &self.cells[*peer])
                    .any(|peer| peer.value > 0 && peer.value == cell.value)
        })
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cells.iter().map(|cell| cell.value.to_string()).join(""))
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        self.cells
            .iter()
            .zip(other.cells.iter())
            .all(|(a, b)| a.value == b.value)
    }
}

#[derive(PartialEq, Error, Debug)]
pub enum ParseError {
    #[error("Invalid digit '{0}' at index {1} while parsing sudoku")]
    InvalidDigit(char, usize),
    #[error("Invalid sudoku at index '{0}'")]
    InvalidSudoku(usize),
}

impl FromStr for Grid {
    type Err = Box<dyn Error + Sync + Send>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::new();
        for (index, c) in string.char_indices() {
            let value = c.to_digit(10).ok_or(ParseError::InvalidDigit(c, index))? as u8;
            if !grid.set(index, value, true) {
                return Err(Box::new(ParseError::InvalidSudoku(index)));
            }
            let cell = &mut grid.cells[index];
            if cell.value != 0 {
                cell.frozen = true;
            }
        }
        Ok(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peers() {
        let peers = PEERS[0];
        assert_eq!(
            peers,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 18, 27, 36, 45, 54, 63, 72, 10, 11, 19, 20]
        );
    }

    #[test]
    fn error_digit() {
        let result =
            "a60000300400700000000000080000008012500600000000000050082000700000500600000010000".parse::<Grid>();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().downcast::<ParseError>().unwrap().as_ref(),
            &ParseError::InvalidDigit('a', 0)
        )
    }

    #[test]
    fn error_invalid() {
        let result =
            "660000300400700000000000080000008012500600000000000050082000700000500600000010000".parse::<Grid>();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().downcast::<ParseError>().unwrap().as_ref(),
            &ParseError::InvalidSudoku(1)
        )
    }
}
