pub mod conway;

use std::{thread::{sleep}, time::Duration};

use sparse_bin_mat::SparseBinMat;

pub mod view;

#[macroquad::main("Rusty Conway")]
async fn main() {
    let n = 5;
    
    let rows = vec![vec![0,1,2,3,4],vec![0,4],vec![0,2,4],vec![0,4],vec![0,1,2,3,4]];
    let mut grid = SparseBinMat::new(n,rows);

    loop {
        view::draw(n, &grid).await;
        grid = conway::tick(&grid);
        sleep(Duration::from_millis(200));
    }
}
