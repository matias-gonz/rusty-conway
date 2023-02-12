use sparse_bin_mat::SparseBinMat;
use sparse_bin_mat::BinNum;

pub fn tick(grid: &SparseBinMat) -> SparseBinMat{
    let n = grid.number_of_rows();
    let mut new_grid = SparseBinMat::zeros(n,n);

    for i in 0..n {
        for j in 0..n {
            let cell = grid.get(i, j).unwrap();
            match cell_is_alive(cell) {
                true => {}
                false => {}
            }
        }
    }
    new_grid
}

fn cell_is_alive(cell: BinNum) -> bool {
    cell.is_one()
}

fn cell_is_underpopulated(x: usize, y: usize, grid: SparseBinMat) -> bool {
    cell_alive_neighbour_count(x, y, grid) < 2
}

fn cell_is_overpopulated(x: usize, y: usize, grid: SparseBinMat) -> bool {
    cell_alive_neighbour_count(x, y, grid) > 3
}

fn cell_alive_neighbour_count(x: usize, y: usize, grid: SparseBinMat) -> usize{
    let mut count = 0;
    for i in 0 ..= 2{
        for j in 0 ..= 2{
            if x + i == 0 || y + j == 0 { continue }
            let cell = grid.get(x+i-1, y+j-1);
            match cell {
                None => {},
                Some(c) => {
                    if cell_is_alive(c) { count += 1}
                }
            }
        }
    }
    if cell_is_alive(grid.get(x,y).unwrap()) { count -= 1}
    count
}


#[cfg(test)]
mod tests {
    use crate::conway::*;
    use sparse_bin_mat::SparseBinMat;
    
    fn get_empty_grid(n: usize) -> SparseBinMat {
        SparseBinMat::zeros(n,n)
    }

    #[test]
    fn empty_grid_should_stay_empty_after_tick() {
        let grid = get_empty_grid(1);
        assert_eq!(grid,tick(&grid));
    }

    #[test]
    fn cell_should_die_when_it_is_alone() {
        let grid_one_cell = SparseBinMat::new(1,vec![vec![0]]);
        let empty_grid = get_empty_grid(1);
        assert_eq!(empty_grid,tick(&grid_one_cell));
    }

    #[test]
    fn two_cells_should_die() {
        let grid_one_cell = SparseBinMat::new(2,vec![vec![0],vec![0]]);
        let empty_grid = get_empty_grid(2);
        assert_eq!(empty_grid,tick(&grid_one_cell));
    }

    #[test]
    fn cell_is_alive_when_it_is_one() {
        let cell = SparseBinMat::new(1,vec![vec![0]]).get(0,0).unwrap();
        assert!(cell_is_alive(cell));
    }

    #[test]
    fn cell_has_no_alive_neighbours_when_it_is_the_only_in_the_grid() {
        let grid = SparseBinMat::new(1,vec![vec![0]]);
        assert_eq!(cell_alive_neighbour_count(0,0,grid), 0);
    }

    #[test]
    fn cell_has_no_alive_neighbours_when_it_is_far_from_others() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert_eq!(cell_alive_neighbour_count(2,2,grid), 0);
    }

    #[test]
    fn cell_has_one_alive_neighbour() {
        let rows = vec![vec![1],vec![1],vec![]];
        let grid = SparseBinMat::new(3,rows);
        assert_eq!(cell_alive_neighbour_count(1,1,grid), 1);
    }
    
    #[test]
    fn cell_has_two_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert_eq!(cell_alive_neighbour_count(0,0,grid), 2);
    }

    #[test]
    fn cell_has_six_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert_eq!(cell_alive_neighbour_count(1,1,grid), 6);
    }

    #[test]
    fn cell_is_underpopulated_when_it_has_one_neighbour() {
        let rows = vec![vec![1],vec![1],vec![]];
        let grid = SparseBinMat::new(3,rows);
        assert!(cell_is_underpopulated(1,1,grid));
    }

    #[test]
    fn cell_is_underpopulated_when_it_has_no_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(cell_is_underpopulated(2,2,grid));
    }

    #[test]
    fn cell_is_not_underpopulated_when_it_has_two_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(!cell_is_underpopulated(0,0,grid));
    }

    #[test]
    fn cell_is_not_overpopulated_when_it_has_two_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(!cell_is_overpopulated(0,0,grid));
    }

    #[test]
    fn cell_is_overpopulated_when_it_has_six_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(cell_is_overpopulated(1,1,grid));
    }
}

