use sparse_bin_mat::SparseBinMat;

pub fn tick(grid: &SparseBinMat) -> SparseBinMat{
    grid.clone()
}


#[cfg(test)]
mod tests {
    use crate::conway;
    use sparse_bin_mat::SparseBinMat;
    
    #[test]
    fn conway_an_empty_grid_should_stay_empty_after_tick() {
        let grid = SparseBinMat::new(1,vec![]);
        assert_eq!(grid,conway::tick(&grid));
    }
}

