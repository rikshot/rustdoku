use std::{error::Error, time::Instant};

use rustdoku_sudoku::solver::alx_solve;
use rustdoku_sudoku::{generator, grid::Grid};

use rayon::prelude::*;

#[cfg(not(target_family = "wasm"))]
use mimalloc::MiMalloc;

#[cfg(not(target_family = "wasm"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use clap::{App, Arg, app_from_crate};

type SolveError = Box<dyn Error + Sync + Send>;

fn solve_file(path: &str) -> Result<(), SolveError> {
    let sudoku_file = std::fs::read_to_string(path)?;
    let count = sudoku_file.lines().count();
    println!(
        "Solving {} sudoku{} from '{}'",
        count,
        if count == 1 { "" } else { "s" },
        path
    );
    let start = Instant::now();
    let results: Vec<Result<(Grid, Vec<Grid>), SolveError>> = sudoku_file
        .par_lines()
        .map(|sudoku: &str| -> Result<(Grid, Vec<Grid>), SolveError> {
            let grid: Grid = sudoku.parse()?;
            Ok((grid, alx_solve(&grid, 0)))
        })
        .collect();
    let duration = start.elapsed().as_secs_f32();
    for result in &results {
        match result {
            Ok((grid, solutions)) => {
                for sudoku in solutions {
                    println!("{},{}", grid, sudoku);
                }
            }
            Err(error) => println!("{}", error),
        };
    }
    println!(
        "Solved {} sudoku{} in {}s, ~{}μs per sudoku",
        count,
        if count == 1 { "" } else { "s" },
        duration,
        duration / (count as f32) * 1000000.0
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
    let start = Instant::now();
    let generated = (0..count)
        .into_par_iter()
        .map(|_| generator::generate(givens))
        .collect::<Vec<Grid>>();
    let duration = start.elapsed().as_secs_f32();
    for sudoku in &generated {
        println!("{}", sudoku);
    }
    println!(
        "Generated {} unique sudoku{} with {} givens in {}s, ~{}μs per sudoku",
        count,
        if count == 1 { "" } else { "s" },
        givens,
        duration,
        duration / (count as f32) * 1000000.0
    );
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let matches = app_from_crate!()
        .subcommand(
            App::new("solve").about("Solves given sudoku").arg(
                Arg::new("sudoku_or_path")
                    .help("A sudoku or a path to a file containing sudokus")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            App::new("generate")
                .about("Generates a random sudoku")
                .arg(
                    Arg::new("givens")
                        .help("How many givens are included (default 28)")
                        .index(1),
                )
                .arg(
                    Arg::new("count")
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
        let givens = matches.value_of_t("givens").unwrap_or(28);
        let count = matches.value_of_t("count").unwrap_or(1);
        generate(givens, count);
    }
    Ok(())
}
