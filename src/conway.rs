use sparse_bin_mat::SparseBinMat;

pub fn tick(grid: &SparseBinMat) -> SparseBinMat{
    let n = grid.number_of_rows();
    let mut new_grid = SparseBinMat::zeros(n,n);

    for i in 0..n {
        for j in 0..n {
            let new_cell = get_new_cell(i,j,grid);
            new_grid = new_grid.emplace_at(new_cell, i, j);
        }
    }
    new_grid
}

fn get_new_cell(i: usize, j: usize, grid: &SparseBinMat) -> u8 {
    let mut new_cell = 0;
    match cell_is_alive(i, j, grid) {
        true => {
            if cell_lives_onto_next_generation(i, j, grid){
                new_cell = 1;
            }
        }
        false => {
            if cell_can_reproduce(i, j, grid){
                new_cell = 1;
            }
        }
    }
    new_cell
}

pub fn cell_is_alive(i: usize, j: usize, grid: &SparseBinMat) -> bool {
    match grid.get(i, j) {
        None => {false},
        Some(c) => {
            c.is_one()
        }
    }
}

fn cell_can_reproduce(x: usize, y: usize, grid: &SparseBinMat) -> bool {
    cell_alive_neighbour_count(x, y, grid) == 3
}

fn cell_lives_onto_next_generation(x: usize, y: usize, grid: &SparseBinMat) -> bool {
    let count = cell_alive_neighbour_count(x, y, grid);
    count == 2 || count == 3
}

fn cell_alive_neighbour_count(x: usize, y: usize, grid: &SparseBinMat) -> usize{
    let mut count = 0;
    for i in 0 ..= 2{
        for j in 0 ..= 2{
            if x + i == 0 || y + j == 0 { continue }
            if cell_is_alive(x+i-1, y+j-1, grid) { count += 1}
        }
    }
    if cell_is_alive(x,y,grid) { count -= 1}
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
        let grid_two_cell = SparseBinMat::new(2,vec![vec![0],vec![0]]);
        let empty_grid = get_empty_grid(2);
        assert_eq!(empty_grid,tick(&grid_two_cell));
    }

    #[test]
    fn four_cell_square_should_live() {
        let grid = SparseBinMat::new(2,vec![vec![0,1],vec![0,1]]);
        assert_eq!(grid,tick(&grid));
    }

    #[test]
    fn three_cell_strip_should_rotate() {
        let rows = vec![vec![],vec![],vec![1,2,3],vec![],vec![]];
        let grid = SparseBinMat::new(5,rows);
        let updated_rows = vec![vec![],vec![2],vec![2],vec![2],vec![]];
        let updated_grid = SparseBinMat::new(5, updated_rows);
        assert_eq!(updated_grid, tick(&grid));
    }

    #[test]
    fn cell_is_alive_when_it_is_one() {
        let grid = SparseBinMat::new(1,vec![vec![0]]);
        assert!(cell_is_alive(0, 0, &grid));
    }

    #[test]
    fn cell_has_no_alive_neighbours_when_it_is_the_only_in_the_grid() {
        let grid = SparseBinMat::new(1,vec![vec![0]]);
        assert_eq!(cell_alive_neighbour_count(0,0,&grid), 0);
    }

    #[test]
    fn cell_has_no_alive_neighbours_when_it_is_far_from_others() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert_eq!(cell_alive_neighbour_count(2,2,&grid), 0);
    }

    #[test]
    fn cell_has_one_alive_neighbour() {
        let rows = vec![vec![1],vec![1],vec![]];
        let grid = SparseBinMat::new(3,rows);
        assert_eq!(cell_alive_neighbour_count(1,1,&grid), 1);
    }
    
    #[test]
    fn cell_has_two_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert_eq!(cell_alive_neighbour_count(0,0,&grid), 2);
    }

    #[test]
    fn cell_has_six_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert_eq!(cell_alive_neighbour_count(1,1,&grid), 6);
    }

    #[test]
    fn cell_can_reproduce_when_it_has_three_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,1,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(cell_can_reproduce(0,0,&grid));
    }

    #[test]
    fn cell_lives_when_it_has_three_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,1,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(cell_lives_onto_next_generation(0,0,&grid));
    }

    #[test]
    fn cell_doesnt_live_when_it_has_six_alive_neighbours() {
        let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
        let grid = SparseBinMat::new(5,rows);
        assert!(!cell_lives_onto_next_generation(1,1,&grid));
    }
}

