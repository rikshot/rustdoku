use std::collections::HashSet;

use super::solver::alx_solve;

use super::grid::Grid;

use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate(givens: usize) -> Grid {
    let mut rng = thread_rng();
    loop {
        let mut grid = Grid::new();
        let mut seed = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        seed.shuffle(&mut rng);
        for (i, n) in seed.iter().enumerate() {
            grid.set(i, *n as u8);
        }
        grid = alx_solve(&grid, 1)[0];
        let mut not_removed = (0..81).collect::<HashSet<usize>>();
        let mut stuck = false;
        'outer: while not_removed.len() > givens {
            let mut tried = HashSet::new();
            loop {
                let index = *not_removed.iter().choose(&mut rng).unwrap();
                let cell = grid.get(index);
                let old_value = cell.value();
                grid.set(index, 0);
                if alx_solve(&grid, 2).len() == 1 {
                    not_removed.remove(&index);
                    break;
                } else {
                    grid.set(index, old_value);
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
        for index in not_removed {
            grid.get_mut(index).freeze();
        }
        break grid;
    }
}
