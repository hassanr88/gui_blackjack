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

it also has a way to enter in a bet and actually bet on the black jack and win cash as well as lose cash, lose all your cash, and the game shows an end screen & closes.

*/

mod ui;
mod utils;
use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "gui_blackjack".to_string(),
        window_width: 1050,
        window_height: 1000,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tm = TextureManager::new();
    tm.preload_all(&["assets/card_front.png", "assets/card_back.png"]).await;
    let img_card = StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 100.0, 200.0, 200.0, 60.0, true, 1.0);
    let lbl_title = Label::new("Blackjack", 400.0, 150.0, 60);
    let lbl_money = Label::new("Cash: $100.00", 150.0, 150.0, 30);
    let lbl_dealerscore = Label::new("Dealer Hand (?)", 210.0, 235.0, 40);
    let lbl_playerscore = Label::new("Player Hand ()", 210.0, 535.0, 40);
    let mut txt_bet = TextInput::new(200.0, 900.0, 300.0, 50.0, 25.0);
    let mut btn_hit = TextButton::new(350.0, 750.0, 150.0, 50.0, "Hit", GOLD, DARKBROWN, 30);
    let mut btn_stand = TextButton::new(550.0, 750.0, 150.0, 50.0, "Stand", GOLD, DARKBROWN, 30);
    let mut btn_exit = TextButton::new(750.0, 900.0, 150.0, 50.0, "Exit", RED, BLACK, 30);
    let mut btn_restart = TextButton::new(750.0, 750.0, 150.0, 50.0, "Restart", ORANGE, BLACK, 30);
    let mut btn_deal = TextButton::new(150.0, 750.0, 150.0, 50.0, "Deal", BLUE, DARKBLUE, 30);
    txt_bet
        .set_prompt("Enter in bet amount...")
        .set_allowed_chars("1234567890.")
        .set_max_chars(26);
    btn_deal.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_exit.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_restart.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_stand.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_hit.with_text_color(BLACK).with_hover_text_color(WHITE);
    loop {
        clear_background(DARKGREEN);
        draw_rectangle(100.0, 50.0, 850.0, 800.0, GREEN);
        draw_rectangle(200.0, 200.0, 650.0, 200.0, DARKGREEN);
        draw_rectangle(200.0, 500.0, 650.0, 200.0, DARKGREEN);
        if btn_hit.click() {}
        if btn_stand.click() {}
        if btn_exit.click() {}
        if btn_restart.click() {}
        if btn_deal.click() {}
        //draw_grid(50.0, WHITE);
        lbl_money.draw();
        txt_bet.draw();
        lbl_title.draw();
        lbl_dealerscore.draw();
        lbl_playerscore.draw();
        next_frame().await;
    }
}
