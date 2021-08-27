use crate::grid::{BOXES, COLUMNS, ROWS};

use super::grid::Grid;
use super::solver::alx_solve;

use ahash::AHashSet;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn seed_grid() -> Grid {
    let mut rng = thread_rng();
    let mut grid = Grid::new();
    let mut indices = *[&ROWS, &COLUMNS, &BOXES].iter().choose(&mut rng).unwrap().choose(&mut rng).unwrap();
    indices.shuffle(&mut rng);
    for (n, index) in indices.iter().enumerate() {
        grid.set(*index, n as u8 + 1, false);
    }
    grid
}

pub fn generate(givens: usize) -> Grid {
    assert!((17..=81).contains(&givens), "Givens must be between 17 and 81");
    let mut rng = thread_rng();
    loop {
        let mut grid = seed_grid();
        grid = alx_solve(&grid, 1)[0];
        let mut not_removed = (0..81).collect::<AHashSet<usize>>();
        let mut stuck = false;
        'outer: while not_removed.len() > givens {
            let mut tried = AHashSet::new();
            loop {
                let index = *not_removed.iter().choose(&mut rng).unwrap();
                let old_value = grid.get(index);
                grid.set(index, 0, false);
                if alx_solve(&grid, 2).len() == 1 {
                    not_removed.remove(&index);
                    break;
                } else {
                    grid.set(index, old_value, false);
                    tried.insert(index);
                    if tried == not_removed {
                        stuck = true;
                        break 'outer;
                    }
                }
            }
        }
        if stuck {
            continue;
        }
        for index in 0..81 {
            if not_removed.contains(&index) {
                grid.freeze(index);
            } else {
                grid.set(index, 0, true);
            }
        }
        break grid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_test() {
        let givens = 25;
        let grid = generate(givens);
        assert_eq!(grid.givens(), givens);
    }
}
