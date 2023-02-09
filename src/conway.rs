use sparse_bin_mat::SparseBinMat;

pub fn tick(grid: &SparseBinMat) -> SparseBinMat{
    let n = grid.number_of_rows();
    SparseBinMat::zeros(n,n)
}


#[cfg(test)]
mod tests {
    use crate::conway;
    use sparse_bin_mat::SparseBinMat;
    
    fn get_empty_grid(n: usize) -> SparseBinMat {
        SparseBinMat::zeros(n,n)
    }

    #[test]
    fn empty_grid_should_stay_empty_after_tick() {
        let grid = get_empty_grid(1);
        assert_eq!(grid,conway::tick(&grid));
    }

    #[test]
    fn cell_should_die_when_it_is_alone() {
        let grid_one_cell = SparseBinMat::new(1,vec![vec![0]]);
        let empty_grid = get_empty_grid(1);
        assert_eq!(empty_grid,conway::tick(&grid_one_cell));
    }
}

