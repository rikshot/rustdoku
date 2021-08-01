use crate::sudoku::candidates::Candidates;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct FastCell {
    pub value: u8,
    pub candidates: Candidates,
    pub frozen: bool,
}

pub struct FastGrid {
    pub cells: Box<[FastCell; 81]>,
    rows: Box<[[usize; 9]; 9]>,
    columns: Box<[[usize; 9]; 9]>,
    boxes: Box<[[usize; 9]; 9]>,
    peers: Box<[[usize; 20]; 81]>,
}

impl FastGrid {
    pub fn new() -> Self {
        let mut rows = [[0; 9]; 9];
        let mut columns = [[0; 9]; 9];
        let mut boxes = [[0; 9]; 9];
        let mut peers = [[0; 20]; 81];

        for index in 0..81 {
            let row_index = index / 9;
            let column_index = index % 9;
            let box_index = row_index / 3 * 3 + column_index / 3;
            rows[row_index][0] = index;
            rows[row_index].rotate_left(1);
            columns[column_index][0] = index;
            columns[column_index].rotate_left(1);
            boxes[box_index][0] = index;
            boxes[box_index].rotate_left(1);
        }

        for index in 0..81 {
            let row_index = index / 9;
            let column_index = index % 9;
            let box_index = row_index / 3 * 3 + column_index / 3;
            for peer in rows[row_index] {
                if peer != index {
                    peers[index][0] = peer;
                    peers[index].rotate_left(1);
                }
            }
            for peer in columns[column_index] {
                if peer != index {
                    peers[index][0] = peer;
                    peers[index].rotate_left(1);
                }
            }
            for peer in boxes[box_index] {
                let peer_row_index = peer / 9;
                let peer_column_index = peer % 9;
                if peer_row_index != row_index && peer_column_index != column_index {
                    peers[index][0] = peer;
                    peers[index].rotate_left(1);
                }
            }
        }

        FastGrid {
            cells: Box::new([FastCell { value: 0, candidates: Candidates::new(true), frozen: false }; 81]),
            rows: Box::new(rows),
            columns: Box::new(columns),
            boxes: Box::new(boxes),
            peers: Box::new(peers),
        }
    }

    pub fn set(&self, index: usize, value: u8) -> Option<Self> {
        let mut cells = self.cells.clone();
        if !self.cells[index].frozen {
            if value > 0 {
                cells[index] = FastCell { value, candidates: Candidates::new(false), frozen: false };
                let cell = cells[index];
                for peer in self.peers[index] {
                    let peer = &mut cells[peer];
                    if peer.value == 0 {
                        peer.candidates.unset((cell.value - 1) as usize);
                        if peer.candidates.none() {
                            return None;
                        }
                    }
                }
            } else {
                cells[index] = FastCell { value: 0, candidates: Candidates::new(true), frozen: false };
                for peer in self.peers[index] {
                    let peer = &self.cells[peer];
                    if peer.value > 0 {
                        cells[index].candidates.unset((peer.value - 1) as usize);
                        if self.cells[index].candidates.none() {
                            return None;
                        }
                    }
                }
            }
            return Some(FastGrid {
                cells,
                rows: self.rows.clone(),
                columns: self.columns.clone(),
                boxes: self.boxes.clone(),
                peers: self.peers.clone(),
            });
        }
        None
    }

    pub fn is_complete(&self) -> bool {
        !self.cells.iter().any(|cell| cell.value == 0)
    }

    pub fn is_valid(&self) -> bool {
        !self.cells.iter().enumerate().any(|(index, cell)| {
            cell.value > 0
                && self
                    .peers[index].iter()
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
    fn new_grid() {
        let grid = FastGrid::new();
        let peers = grid.peers[0];
        assert_eq!(
            peers,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 18, 27, 36, 45, 54, 63, 72, 10, 11, 19, 20]
        );
    }
}
