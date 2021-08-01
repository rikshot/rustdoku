use crate::sudoku::candidates::Candidates;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct FastCell {
    pub value: u8,
    pub candidates: Candidates,
    pub frozen: bool,
}

pub struct FastGrid {
    pub cells: [FastCell; 81],
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

const fn peers() -> [[usize; 20]; 81] {
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

    peers
}

static PEERS: [[usize; 20]; 81] = peers();

impl FastGrid {
    pub fn new() -> Self {
        FastGrid {
            cells: [FastCell {
                value: 0,
                candidates: Candidates::new(true),
                frozen: false,
            }; 81],
        }
    }

    pub fn set(&self, index: usize, value: u8) -> Option<Self> {
        let mut cells = self.cells;
        if !cells[index].frozen {
            if value > 0 {
                let cell = FastCell {
                    value,
                    candidates: Candidates::new(false),
                    frozen: false,
                };
                for peer in PEERS[index] {
                    let peer = &mut cells[peer];
                    if peer.value == 0 {
                        peer.candidates.unset((cell.value - 1) as usize);
                        if peer.candidates.none() {
                            return None;
                        }
                    }
                }
                cells[index] = cell;
            } else {
                let mut cell = FastCell {
                    value: 0,
                    candidates: Candidates::new(true),
                    frozen: false,
                };
                for peer in PEERS[index] {
                    let peer = &cells[peer];
                    if peer.value > 0 {
                        cell.candidates.unset((peer.value - 1) as usize);
                        if cell.candidates.none() {
                            return None;
                        }
                    }
                }
                cells[index] = cell;
            }
            return Some(FastGrid { cells });
        }
        None
    }

    pub fn is_complete(&self) -> bool {
        !self.cells.iter().any(|cell| cell.value == 0)
    }

    pub fn is_valid(&self) -> bool {
        !self.cells.iter().enumerate().any(|(index, cell)| {
            cell.value > 0
                && PEERS[index]
                    .iter()
                    .any(|peer| self.cells[*peer].value > 0 && self.cells[*peer].value == cell.value)
        })
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for cell in self.cells.iter() {
            string.push_str(cell.value.to_string().as_str())
        }
        string
    }
}

impl PartialEq for FastGrid {
    fn eq(&self, other: &FastGrid) -> bool {
        for (a, b) in self.cells.iter().zip(other.cells.iter()) {
            if a.value != b.value {
                return false;
            }
        }
        true
    }
}

impl FromStr for FastGrid {
    type Err = std::num::ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut grid = FastGrid::new();
        for (index, c) in string.char_indices() {
            grid = grid.set(index, c.to_digit(10).unwrap() as u8).unwrap();
            let cell = &mut grid.cells[index];
            if cell.value != 0 {
                cell.frozen = true;
                cell.candidates.unset_all();
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
}
