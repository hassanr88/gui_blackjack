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
use miniquad::date;

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
    rand::srand(date::now() as u64);
    let numcard = rand::gen_range(1, 14);
    let group = rand::gen_range(1, 4);
    let mut player_cards: Vec<StillImage> = vec![
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 250.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 500.0, true, 1.0),
    ];
    let mut dealer_cards: Vec<StillImage> = vec![
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 150.0, 200.0, 250.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_front.png").unwrap(), 150.0, 200.0, 450.0, 200.0, true, 1.0),
    ];
    let lbl_title = Label::new("Blackjack", 400.0, 150.0, 60);
    let mut lbl_money = Label::new("Cash: $100.00", 150.0, 100.0, 30);
    let lbl_dealerscore = Label::new("Dealer Hand (?)", 210.0, 185.0, 40);
    let lbl_playerscore = Label::new("Player Hand ()", 210.0, 485.0, 40);
    let mut txt_bet = TextInput::new(200.0, 900.0, 300.0, 50.0, 25.0);
    let mut btn_hit = TextButton::new(350.0, 750.0, 150.0, 50.0, "Hit", GOLD, DARKBROWN, 30);
    let mut btn_stand = TextButton::new(550.0, 750.0, 150.0, 50.0, "Stand", GOLD, DARKBROWN, 30);
    let mut btn_exit = TextButton::new(750.0, 900.0, 150.0, 50.0, "Exit", RED, BLACK, 30);
    let mut btn_restart = TextButton::new(750.0, 750.0, 150.0, 50.0, "Restart", ORANGE, BLACK, 30);
    let mut btn_deal = TextButton::new(150.0, 750.0, 150.0, 50.0, "Deal", BLUE, DARKBLUE, 30);
    let mut account = 100.0;
    let mut game_start = false;
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
        if btn_exit.click() {
            break;
        }
        if btn_restart.click() {
            btn_deal.enabled = true;
            txt_bet.set_enabled(true);
            game_start = false;
            account = 100.0;
            lbl_money.set_text(format!("Cash: ${:.2}", account));
        }
        if btn_deal.click() {
            let bet = txt_bet.get_text().trim().parse().unwrap_or(0.0);
            if bet <= account && bet > 0.0 {
                let n = account - bet;
                lbl_money.set_text(format!("Cash: ${:.2}", n));
                txt_bet.set_enabled(false);
                account = n;
                btn_deal.enabled = false;
                game_start = true;
            } else {
            }
        }
        //draw_grid(50.0, WHITE);
        lbl_money.draw();
        if game_start {
            for i in 0..player_cards.len() {
                player_cards[i].draw();
            }
            for i in 0..dealer_cards.len() {
                dealer_cards[i].draw();
            }
        }
        txt_bet.draw();
        lbl_title.draw();
        lbl_dealerscore.draw();
        lbl_playerscore.draw();
        next_frame().await;
    }
}