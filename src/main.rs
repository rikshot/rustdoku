extern crate rustdoku;

use rustdoku::sudoku::fast_solver::brute_force;

fn main() {
    let sudoku_file = std::fs::read_to_string("sudoku17").expect("sudoku17 file not found");

    for sudoku in sudoku_file.lines().take(50) {
        let mut grid = sudoku.parse().expect("could not parse sudoku");
        grid = brute_force(grid);
        println!("{} = {} = {}", sudoku, grid.to_string(), grid.is_valid());
    }
}
