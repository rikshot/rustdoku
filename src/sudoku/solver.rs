use std::collections::{HashMap, HashSet};

use itertools::iproduct;

use super::grid::Grid;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Constraint {
    RC,
    RN,
    CN,
    BN,
}

type ConstraintType = (Constraint, (u8, u8));
type RCNType = (u8, u8, u8);

pub fn alx_solve(grid: &Grid, limit: usize) -> Vec<Grid> {
    let x: Vec<ConstraintType> = iproduct!(0..9, 0..9)
        .map(|rc| (Constraint::RC, rc))
        .chain(iproduct!(0..9, 1..10).map(|rn| (Constraint::RN, rn)))
        .chain(iproduct!(0..9, 1..10).map(|cn| (Constraint::CN, cn)))
        .chain(iproduct!(0..9, 1..10).map(|bn| (Constraint::BN, bn)))
        .collect();

    let mut y = HashMap::with_capacity(729);
    for (r, c, n) in iproduct!(0..9, 0..9, 1..10) {
        let b = r / 3 * 3 + c / 3;
        y.insert(
            (r, c, n),
            [
                (Constraint::RC, (r, c)),
                (Constraint::RN, (r, n)),
                (Constraint::CN, (c, n)),
                (Constraint::BN, (b, n)),
            ],
        );
    }

    let mut x = exact_cover(&x, &y);

    for row in 0..9 {
        for column in 0..9 {
            let cell = grid.get(row * 9 + column);
            if cell.value() > 0 {
                select(&mut x, &y, (row as u8, column as u8, cell.value()));
            }
        }
    }

    let solutions = solve(&mut x, &y, &mut vec![], limit);

    solutions
        .iter()
        .map(|solution| {
            let mut grid = *grid;
            for (r, c, n) in solution {
                grid.set((r * 9 + c) as usize, *n);
            }
            grid
        })
        .collect()
}

fn exact_cover(
    x: &[ConstraintType],
    y: &HashMap<RCNType, [ConstraintType; 4]>,
) -> HashMap<ConstraintType, HashSet<RCNType>> {
    let mut ec: HashMap<ConstraintType, HashSet<RCNType>> = HashMap::with_capacity(324);
    for j in x.iter() {
        ec.insert(*j, HashSet::with_capacity(9));
    }
    for (i, row) in y.iter() {
        for j in *row {
            ec.get_mut(&j).unwrap().insert(*i);
        }
    }
    ec
}

fn solve(
    x: &mut HashMap<ConstraintType, HashSet<RCNType>>,
    y: &HashMap<RCNType, [ConstraintType; 4]>,
    solution: &mut Vec<RCNType>,
    limit: usize,
) -> Vec<Vec<RCNType>> {
    if x.is_empty() {
        return vec![solution.clone()];
    }
    let (c, _) = x.iter().min_by_key(|(_k, v)| v.len()).unwrap();
    let mut solutions = vec![];
    for r in x[c].iter().cloned().collect::<Vec<RCNType>>() {
        solution.push(r);
        let mut cols = select(x, y, r);
        solutions.append(&mut solve(x, y, solution, limit));
        deselect(x, y, r, &mut cols);
        solution.pop();
        if limit > 0 && solutions.len() == limit {
            break;
        }
    }
    solutions
}

fn select(
    x: &mut HashMap<ConstraintType, HashSet<RCNType>>,
    y: &HashMap<RCNType, [ConstraintType; 4]>,
    r: RCNType,
) -> Vec<HashSet<RCNType>> {
    let mut cols: Vec<HashSet<RCNType>> = vec![];
    for j in y[&r] {
        let mut remove_set = vec![];
        for i in &x[&j] {
            for k in y[i] {
                if k != j {
                    remove_set.push((k, *i));
                }
            }
        }
        for (k, i) in remove_set {
            x.get_mut(&k).unwrap().remove(&i);
        }
        cols.push(x.remove(&j).unwrap());
    }
    cols
}

fn deselect(
    x: &mut HashMap<ConstraintType, HashSet<RCNType>>,
    y: &HashMap<RCNType, [ConstraintType; 4]>,
    r: RCNType,
    cols: &mut Vec<HashSet<RCNType>>,
) {
    for j in y[&r].iter().rev() {
        x.insert(*j, cols.pop().unwrap());
        let mut insert_set = vec![];
        for i in &x[j] {
            for k in y[i] {
                if k != *j {
                    insert_set.push((k, *i));
                }
            }
        }
        for (k, i) in insert_set {
            x.get_mut(&k).unwrap().insert(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alx_solver_test() {
        let grid = "060000300400700000000000080000008012500600000000000050082000700000500600000010000"
            .parse()
            .unwrap();
        let complete_grid: Grid = "961845327458723169237169584796358412524691873813274956182436795379582641645917238"
            .parse()
            .unwrap();
        let grids = &alx_solve(&grid, 0);
        assert_eq!(grids.len(), 1);
        assert_eq!(
            grids[0],
            complete_grid,
            "grid = {}, complete_grid = {}",
            grid.to_string(),
            complete_grid.to_string()
        )
    }

    #[test]
    fn alx_solver_multiple_solutions_test() {
        let grid = "060000000400700000000000080000008012500600000000000050082000700000500600000010000"
            .parse()
            .unwrap();
        let grid = &alx_solve(&grid, 0);
        assert!(grid.len() > 1);
    }
}
