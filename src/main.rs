extern crate rustdoku;

use rustdoku::sudoku::grid::Grid;

fn main() {
	let mut grid = Grid::from_str(
		"000000010400000000020000000000050407008000300001090000300400200050100000000806000",
	);
	println!("{}", grid.pretty());
	grid.brute_force();	
	println!("{}", grid.pretty());
}
