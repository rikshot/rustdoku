use super::grid::Grid;

pub fn brute_force(grid: &mut Grid) -> &mut Grid {
    brute_force_impl(grid, 0);
    grid
}

fn brute_force_impl(grid: &mut Grid, index: usize) -> bool {
    if index > 80 {
        return true;
    }
    let cell = grid.cell(index).clone();
    if !cell.borrow().frozen && cell.borrow().value == 0 {
        if !grid.update_candidates(index) {
            return false;
        }
        let candidates = cell.borrow().candidates.clone();
        if candidates.some() {
            for value in 1..10 {
                if cell.borrow().candidates.get(value - 1) {
                    cell.borrow_mut().set(value);
                    if brute_force_impl(grid, index + 1) {
                        return true;
                    }
                    cell.borrow_mut().set(0);
                    cell.borrow_mut().candidates = candidates;
                }
            }
        }
        return false;
    }
    return brute_force_impl(grid, index + 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_test() {
        let mut grid = Grid::from_str("060000300400700000000000080000008012500600000000000050082000700000500600000010000");
        let complete_grid = Grid::from_str("961845327458723169237169584796358412524691873813274956182436795379582641645917238");
        brute_force(&mut grid);
        assert!(grid == complete_grid, "grid = {}, complete_grid = {}", grid.to_string(), complete_grid.to_string())
    }
}
