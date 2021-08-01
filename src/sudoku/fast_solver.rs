use super::fast_grid::FastGrid;

pub fn brute_force(grid: FastGrid) -> FastGrid {
    brute_force_impl(grid, 0).unwrap()
}

fn brute_force_impl(grid: FastGrid, index: usize) -> Option<FastGrid> {
    if index > 80 {
        return Some(grid);
    }
    let cell = &grid.cells[index];
    if !cell.frozen && cell.value == 0 {
        let candidates = cell.candidates;
        if candidates.some() {
            for value in 1..10 {
                if cell.candidates.get(value - 1) {
                    let grid = grid.set(index, value as u8);
                    if grid.is_some() {
                        let grid = brute_force_impl(grid.unwrap(), index + 1);
                        if grid.is_some() {
                            return Some(grid.unwrap());
                        }
                    }
                }
            }
        }
        return None;
    }
    brute_force_impl(grid, index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_test() {
        let mut grid = "060000300400700000000000080000008012500600000000000050082000700000500600000010000"
            .parse()
            .unwrap();
        let complete_grid = "961845327458723169237169584796358412524691873813274956182436795379582641645917238"
            .parse()
            .unwrap();
        grid = brute_force(grid);
        assert!(
            grid == complete_grid,
            "grid = {}, complete_grid = {}",
            grid.to_string(),
            complete_grid.to_string()
        )
    }
}
