use std::path::PathBuf;
use std::{error::Error, time::Instant};

use clap::{arg, command, ArgGroup, Parser, Subcommand};
#[cfg(not(target_family = "wasm"))]
use mimalloc::MiMalloc;
use rayon::prelude::*;

use rustdoku_sudoku::solver::alx_solve;
use rustdoku_sudoku::{generator, grid::Grid};

#[cfg(not(target_family = "wasm"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

type SolveError = Box<dyn Error + Sync + Send>;

fn solve_file(path: &PathBuf, verbose: bool) -> Result<(), SolveError> {
    let sudoku_file = std::fs::read_to_string(path)?;
    let count = sudoku_file.lines().count();
    if verbose {
        println!(
            "Solving {} sudoku{} from '{}'",
            count,
            if count == 1 { "" } else { "s" },
            path.display()
        );
    }
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
    if verbose {
        println!(
            "Solved {} sudoku{} in {}s, ~{}μs per sudoku",
            count,
            if count == 1 { "" } else { "s" },
            duration,
            duration / (count as f32) * 1000000.0
        );
    }
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

fn generate(givens: u8, count: usize, verbose: bool) {
    if verbose {
        println!(
            "Generating {} unique sudoku{} with {} givens",
            count,
            if count == 1 { "" } else { "s" },
            givens
        );
    }
    let start = Instant::now();
    let generated = (0..count)
        .into_par_iter()
        .map(|_| generator::generate(givens as usize))
        .collect::<Vec<Grid>>();
    let duration = start.elapsed().as_secs_f32();
    for sudoku in &generated {
        println!("{}", sudoku);
    }
    if verbose {
        println!(
            "Generated {} unique sudoku{} with {} givens in {}s, ~{}μs per sudoku",
            count,
            if count == 1 { "" } else { "s" },
            givens,
            duration,
            duration / (count as f32) * 1000000.0
        );
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Prints additional statistics
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve sudokus
    #[command(group(ArgGroup::new("input").required(true).args(["sudoku", "path"])))]
    Solve {
        /// Solves a single sudoku
        #[arg(short, long)]
        sudoku: Option<String>,

        /// Solves sudokus from a file
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Generate sudokus
    Generate {
        /// How many givens to generate
        #[arg(short, long, default_value_t = 28, value_parser = clap::value_parser ! (u8).range(17..81))]
        givens: u8,

        /// How many sudokus to generate
        #[arg(short, long, default_value_t = 1)]
        count: usize,
    },
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solve { sudoku, path } => {
            if let Some(sudoku) = sudoku {
                solve_single(&sudoku)
            } else if let Some(path) = path {
                solve_file(&path, cli.verbose)
            } else {
                Ok(())
            }
        }
        Commands::Generate { givens, count } => {
            generate(givens, count, cli.verbose);
            Ok(())
        }
    }
}
