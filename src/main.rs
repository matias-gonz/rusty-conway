pub mod conway;

use std::{thread::{sleep}, time::Duration};

use rand::*;

use sparse_bin_mat::SparseBinMat;

pub mod view;

#[macroquad::main("Rusty Conway")]
async fn main() {
    let n = 25;
    let mut grid = SparseBinMat::zeros(n,n);
    let mut rng = rand::thread_rng();

    for i in 0..n {
        for j in 0..n {
            grid = grid.emplace_at(rng.gen_range(0..=1), i, j);
        }
    }

    loop {
        view::draw(n, &grid).await;
        grid = conway::tick(&grid);
        sleep(Duration::from_millis(50));
    }
}
