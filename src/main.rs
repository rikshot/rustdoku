extern crate rustdoku;

use rustdoku::sudoku::grid::Grid;
use rustdoku::sudoku::solver::brute_force;

fn main() {
	let sudoku_file = std::fs::read_to_string("sudoku17").expect("sudoku17 file not found");
	let sudokus: Vec<&str> = sudoku_file.split("\n").collect();
	for sudoku in sudokus {
		let mut grid = Grid::from_str(sudoku);
		brute_force(&mut grid);
		println!("{} = {}", sudoku, grid.to_string());
	}
}
