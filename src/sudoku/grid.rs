use std::cell::RefCell;
use std::iter::*;
use std::rc::Rc;

use super::cell::Cell;

pub struct Grid {
    cells: Vec<Rc<RefCell<Cell>>>,
}

impl Grid {
    pub fn new() -> Grid {
        let mut cells: Vec<Rc<RefCell<Cell>>> = Vec::with_capacity(81);
        for index in 0..81 {
            cells.push(Rc::new(RefCell::new(Cell::new(index, 0))));
        }
        Grid { cells }
    }

    pub fn from_str(string: &str) -> Grid {
        let mut grid = Grid::new();
        for (index, c) in string.char_indices() {
            let cell = &mut grid.cells[index];
            cell.borrow_mut().set(c.to_digit(10).unwrap() as usize);
            if cell.borrow().value != 0 {
                cell.borrow_mut().frozen = true;
                cell.borrow_mut().candidates.unset_all();
            }
        }
        for index in 0..81 {
            grid.update_candidates(index);
        }
        grid
    }

    pub fn cells(&self) -> impl Iterator<Item = &Rc<RefCell<Cell>>> {
        self.cells.iter()
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut Rc<RefCell<Cell>>> {
        self.cells.iter_mut()
    }

    pub fn get_row(&self, index: usize) -> impl Iterator<Item = &Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(move |cell| cell.borrow().row_index == index)
    }

    pub fn get_column(&self, index: usize) -> impl Iterator<Item = &Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(move |cell| cell.borrow().column_index == index)
    }

    pub fn get_box(&self, index: usize) -> impl Iterator<Item = &Rc<RefCell<Cell>>> {
        self.cells
            .iter()
            .filter(move |cell| cell.borrow().box_index == index)
    }

    pub fn get_peers(&self, index: usize) -> impl Iterator<Item = &Rc<RefCell<Cell>>> {
        let cell = &self.cells[index];
        let rows = self
            .get_row(cell.borrow().row_index)
            .filter(move |peer| peer.borrow().index != index);
        let columns = self
            .get_column(cell.borrow().column_index)
            .filter(move |peer| peer.borrow().index != index);
        let boxes = self.get_box(cell.borrow().box_index).filter(move |peer| {
            peer.borrow().row_index != cell.borrow().row_index
                && peer.borrow().column_index != cell.borrow().column_index
        });
        rows.chain(columns).chain(boxes)
    }

    pub fn is_valid(&self) -> bool {
        !self.cells.iter().any(|cell| {
            cell.borrow().value > 0
                && self.get_peers(cell.borrow().index).any(|peer| {
                    peer.borrow().value > 0 && peer.borrow().value == cell.borrow().value
                })
        })
    }

    pub fn is_complete(&self) -> bool {
        !self.cells.iter().any(|cell| cell.borrow().value == 0)
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for cell in &self.cells {
            string.push_str(cell.borrow().value.to_string().as_str())
        }
        string
    }

    pub fn pretty(&self) -> String {
        let mut string = String::new();
        for row in 0..9 {
            if (row % 3) == 0 {
                string.push_str("+-------+-------+-------+\n");
            }
            for column in 0..9 {
                string.push_str(if (column % 9) == 0 { "| " } else if (column % 3) == 0 { " | " } else { " " });
                string.push_str(self.cells[row * 9 + column].borrow().value.to_string().as_str());
            }
            string.push_str(" |\n");
        }
        string.push_str("+-------+-------+-------+");
        string
    }

    fn update_candidates(&mut self, index: usize) -> bool {
        let cell = &self.cells[index];
        if cell.borrow().value == 0 {
            cell.borrow_mut().candidates.set_all();
            for peer in self.get_peers(index) {
                if peer.borrow().value > 0 {
                    cell.borrow_mut().candidates.unset(peer.borrow().value - 1);
                    if cell.borrow().candidates.none() {
                        return false;
                    }
                }
            }
        } else {
            for peer in self.get_peers(index) {
                if peer.borrow().value == 0 {
                    peer.borrow_mut().candidates.unset(cell.borrow().value - 1);
                    if peer.borrow().candidates.none() {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn brute_force(&mut self) {
        self.brute_force_impl(0);
    }

    fn brute_force_impl(&mut self, index: usize) -> bool {
        if index > 80 {
            return true;
        }
        let cell = self.cells[index].clone();
        if !cell.borrow().frozen && cell.borrow().value == 0 {
            if !self.update_candidates(index) {
                return false;
            }
            let candidates = cell.borrow().candidates.clone();
            if candidates.some() {
                for value in 1..10 {
                    if cell.borrow().candidates.get(value - 1) {
                        cell.borrow_mut().set(value);
                        if self.brute_force_impl(index + 1) {
                            return true;
                        }
                        cell.borrow_mut().set(0);
                        cell.borrow_mut().candidates = candidates;
                    }
                }
            }
            return false;
        }
        return self.brute_force_impl(index + 1);
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        for (a, b) in self.cells().zip(other.cells()) {
            if a.borrow().value != b.borrow().value {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_count() {
        let grid = Grid::new();
        let peers: Vec<&Rc<RefCell<Cell>>> = grid.get_peers(0).collect();
        assert!(peers.len() == 20);
    }

    #[test]
    fn brute_force() {
        let mut grid = Grid::from_str("060000300400700000000000080000008012500600000000000050082000700000500600000010000");
        let complete_grid = Grid::from_str("961845327458723169237169584796358412524691873813274956182436795379582641645917238");
        grid.brute_force();
        assert!(grid == complete_grid, "grid = {}, complete_grid = {}", grid.to_string(), complete_grid.to_string())
    }
}
