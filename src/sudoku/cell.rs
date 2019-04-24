use super::candidates::Candidates;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
	pub value: usize,
	pub index: usize,
	pub row_index: usize,
    pub column_index: usize,
    pub box_index: usize,
    pub candidates: Candidates,
	pub frozen: bool
}

impl Cell {

	pub fn new(index: usize, value: usize) -> Cell {
		let row_index = index / 9;
		let column_index = index % 9;
		let box_index = row_index / 3 * 3 + column_index / 3;
		Cell { 
			value: value,
			index: index,
			row_index: row_index,
			column_index: column_index,
			box_index: box_index,
			candidates: Candidates::new(),
			frozen: false
		}
	}

	pub fn set(&mut self, value: usize) {
		if !self.frozen {
			self.value = value;
			if value > 0 {
				self.candidates.unset_all()
			} else {
				self.candidates.set_all()
			}
		}
	}

}
