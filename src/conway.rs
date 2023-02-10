use sparse_bin_mat::SparseBinMat;
use sparse_bin_mat::BinNum;

pub fn tick(grid: &SparseBinMat) -> SparseBinMat{
    let n = grid.number_of_rows();
    let mut new_grid = SparseBinMat::zeros(n,n);

    for i in 0..n {
        for j in 0..n {
            let cell = grid.get(i, j).unwrap();
            match is_alive(cell) {
                true => {}
                false => {}
            }
        }
    }
    new_grid
}

fn is_alive(cell: BinNum) -> bool {
    cell.is_one()
}


#[cfg(test)]
mod tests {
    use crate::conway::{self, is_alive};
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

    #[test]
    fn two_cells_should_die() {
        let grid_one_cell = SparseBinMat::new(2,vec![vec![0],vec![0]]);
        let empty_grid = get_empty_grid(2);
        assert_eq!(empty_grid,conway::tick(&grid_one_cell));
    }

    #[test]
    fn cell_is_alive_when_it_is_one() {
        let cell = SparseBinMat::new(1,vec![vec![0]]).get(0,0).unwrap();
        assert!(is_alive(cell));
    }
}

