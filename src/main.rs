extern crate rustdoku;

use rustdoku::sudoku::solver::brute_force;

fn main() {
	let sudoku_file = std::fs::read_to_string("sudoku17").expect("sudoku17 file not found");
	let sudokus: Vec<&str> = sudoku_file.lines().collect();
	for sudoku in sudokus {
		let mut grid = sudoku.parse().expect("could not parse sudoku");
		brute_force(&mut grid);
		println!("{} = {} = {}", sudoku, grid.to_string(), grid.is_valid());
	}
}
