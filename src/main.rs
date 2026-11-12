/*
By: Hassan Rana
Date: 2026-11-06
Program Details: 

A basic blackjack game that stores all 52 cards in a list. 
The dealer is given 1 card to start and the player is given two.  
The goal is to get closest to 21 without going over.  
The users is then given the option for Hitting(getting another card) until they want to Stand (stop) or reach 5 cards.  
When are done, the dealer must keep hitting until he gets to a minimum of 16 where he Stands(stops).
The program then checks to see who wins.

*/

mod ui;
mod utils;
use crate::utils::preload_image::TextureManager;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::grid::draw_grid;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "gui_blackjack".to_string(),
        window_width: 1050,
        window_height: 850,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    loop {
        clear_background(DARKGREEN);
        draw_rectangle(100.0, 50.0, 850.0, 750.0, GREEN);
        draw_rectangle(200.0, 200.0, 650.0, 200.0, DARKGREEN);
        draw_rectangle(200.0, 500.0, 650.0, 200.0, DARKGREEN);
        //draw_grid(50.0, WHITE);
        next_frame().await;
    }
}
