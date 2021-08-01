extern crate rustdoku;

use std::time::Instant;

use rustdoku::sudoku::fast_solver::brute_force as brute_force_fast;
use rustdoku::sudoku::solver::brute_force;

fn main() {
    let n = 50;
    let sudoku_file = std::fs::read_to_string("sudoku17").expect("sudoku17 file not found");

    println!("Solving {} first sudokus (slow)...", n);
    let slow = Instant::now();
    for sudoku in sudoku_file.lines().take(n) {
        let mut grid = sudoku.parse().expect("could not parse sudoku");
        brute_force(&mut grid);
        println!("{} = {} = {}", sudoku, grid.to_string(), grid.is_valid());
    }
    let slow = slow.elapsed().as_secs_f32();

    println!("Solving {} first sudokus (fast)...", n);
    let fast = Instant::now();
    for sudoku in sudoku_file.lines().take(n) {
        let mut grid = sudoku.parse().expect("could not parse sudoku");
        grid = brute_force_fast(grid);
        println!("{} = {} = {}", sudoku, grid.to_string(), grid.is_valid());
    }
    let fast = fast.elapsed().as_secs_f32();

    println!("Solving {} first sudokus slowly took: {}s", n, slow);
    println!("Solving {} first sudokus fastly took: {}s", n, fast);
    println!("Same algo, but more efficient data structures equals {}% better perf", f32::round(slow / fast * 100.0));
}
