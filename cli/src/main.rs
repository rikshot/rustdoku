use std::{error::Error, time::Instant};

use rustdoku_sudoku::solver::alx_solve;
use rustdoku_sudoku::{generator, grid::Grid};

use rayon::prelude::*;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, value_t, Arg, SubCommand};

fn solve_file(path: &str) -> Result<(), Box<dyn Error + Sync + Send>> {
    let sudoku_file = std::fs::read_to_string(path)?;
    println!("Solving '{}'", path);
    let start = Instant::now();
    sudoku_file
        .par_lines()
        .try_for_each(|sudoku: &str| -> Result<(), Box<dyn Error + Sync + Send>> {
            let mut grid: Grid = sudoku.parse()?;
            let grids = alx_solve(&grid, 0);
            assert_eq!(grids.len(), 1);
            grid = grids[0];
            println!("{} = {} = {}", sudoku, grid, grid.is_valid());
            Ok(())
        })?;
    let duration = start.elapsed().as_secs_f32();
    let count = sudoku_file.lines().count() as f32;
    println!(
        "Sudokus: {} Duration: {}s, ~{}ms per sudoku",
        count,
        duration,
        duration / count * 1000.0
    );
    Ok(())
}

fn solve_single(sudoku: &str) -> Result<(), Box<dyn Error + Sync + Send>> {
    let grid: Grid = sudoku.parse()?;
    let grids = alx_solve(&grid, 0);
    for grid in grids {
        println!("{}", grid);
    }
    Ok(())
}

fn generate(givens: usize, count: usize) {
    println!(
        "Generating {} unique sudoku{} with {} givens",
        count,
        if count == 1 { "" } else { "s" },
        givens
    );
    for _ in 0..count {
        println!("{}", generator::generate(givens));
    }
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("solve").about("Solves given sudoku").arg(
                Arg::with_name("sudoku_or_path")
                    .help("A sudoku or a path to a file containing sudokus")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates a random sudoku")
                .arg(
                    Arg::with_name("givens")
                        .help("How many givens are included (default 28)")
                        .index(1),
                )
                .arg(
                    Arg::with_name("count")
                        .help("How many sudokus to generate (default 1)")
                        .index(2),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("solve") {
        if let Some(sudoku_or_path) = matches.value_of("sudoku_or_path") {
            match std::fs::metadata(sudoku_or_path) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        solve_file(sudoku_or_path)?;
                    }
                }
                _ => {
                    solve_single(sudoku_or_path)?;
                }
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let givens = value_t!(matches, "givens", usize).unwrap_or(28);
        let count = value_t!(matches, "count", usize).unwrap_or(1);
        generate(givens, count);
    }
    Ok(())
}
