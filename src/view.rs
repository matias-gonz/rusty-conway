use std::ops::Div;

use macroquad::prelude::*;
use sparse_bin_mat::SparseBinMat;

use crate::conway;


pub async fn draw(n: usize, grid: &SparseBinMat) {
    let padding = 50.0;

    clear_background(BLACK);

    for i in 0..n {
        for j in 0..n {
            if(!conway::cell_is_alive(i,j,grid)) { continue;}

            let factor = 0.9;
            let x = (screen_width() - padding).div(n as f32) * i as f32 + padding/2.0;
            let y = (screen_height() - padding).div(n as f32) * j as f32 + padding/2.0;
            let w = (screen_width() - padding).div(n as f32)*factor;
            let h = (screen_height() - padding).div(n as f32)*factor;
            draw_rectangle(x, y, w, h, WHITE);
        }
    }

    next_frame().await
}