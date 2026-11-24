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
use crate::ui::label::{self, Label};
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::TextureManager;
use macroquad::rand::ChooseRandom;
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

fn card_value(card: i32) -> i32 {
    if card >= 48 {
        return 10;
    } else if card >= 44 {
        return 10;
    } else if card >= 40 {
        return 10;
    } else if card >= 36 {
        return 11;
    }
    card / 4 + 2
}

#[macroquad::main(window_conf)]
async fn main() {
    let tm = TextureManager::new();
    tm.preload_with_loading_screen(
        &[
            "assets/2_of_clubs.png",
            "assets/2_of_diamonds.png",
            "assets/2_of_hearts.png",
            "assets/2_of_spades.png",
            "assets/3_of_clubs.png",
            "assets/3_of_diamonds.png",
            "assets/3_of_hearts.png",
            "assets/3_of_spades.png",
            "assets/4_of_clubs.png",
            "assets/4_of_diamonds.png",
            "assets/4_of_hearts.png",
            "assets/4_of_spades.png",
            "assets/5_of_clubs.png",
            "assets/5_of_diamonds.png",
            "assets/5_of_hearts.png",
            "assets/5_of_spades.png",
            "assets/6_of_clubs.png",
            "assets/6_of_diamonds.png",
            "assets/6_of_hearts.png",
            "assets/6_of_spades.png",
            "assets/7_of_clubs.png",
            "assets/7_of_diamonds.png",
            "assets/7_of_hearts.png",
            "assets/7_of_spades.png",
            "assets/8_of_clubs.png",
            "assets/8_of_diamonds.png",
            "assets/8_of_hearts.png",
            "assets/8_of_spades.png",
            "assets/9_of_clubs.png",
            "assets/9_of_diamonds.png",
            "assets/9_of_hearts.png",
            "assets/9_of_spades.png",
            "assets/10_of_clubs.png",
            "assets/10_of_diamonds.png",
            "assets/10_of_hearts.png",
            "assets/10_of_spades.png",
            "assets/ace_of_clubs.png",
            "assets/ace_of_diamonds.png",
            "assets/ace_of_hearts.png",
            "assets/ace_of_spades.png",
            "assets/jack_of_clubs2.png",
            "assets/jack_of_diamonds2.png",
            "assets/jack_of_hearts2.png",
            "assets/jack_of_spades2.png",
            "assets/queen_of_clubs2.png",
            "assets/queen_of_diamonds2.png",
            "assets/queen_of_hearts2.png",
            "assets/queen_of_spades2.png",
            "assets/king_of_clubs2.png",
            "assets/king_of_diamonds2.png",
            "assets/king_of_hearts2.png",
            "assets/king_of_spades2.png",
            "assets/card_back.png",
        ],
        None,
        None,
    )
    .await;
    rand::srand(date::now() as u64);
    let mut player_cards: Vec<StillImage> = vec![
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 205.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 310.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 415.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 520.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 625.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 730.0, 500.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 835.0, 500.0, true, 1.0),
    ];
    let mut dealer_cards: Vec<StillImage> = vec![
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 205.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 310.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 415.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 520.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 625.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 730.0, 200.0, true, 1.0),
        StillImage::from_preload(tm.get_preload("assets/card_back.png").unwrap(), 100.0, 200.0, 835.0, 200.0, true, 1.0),
    ];
    let card_names: Vec<&str> = vec![
        "assets/2_of_clubs.png",
        "assets/2_of_diamonds.png",
        "assets/2_of_hearts.png",
        "assets/2_of_spades.png",
        "assets/3_of_clubs.png",
        "assets/3_of_diamonds.png",
        "assets/3_of_hearts.png",
        "assets/3_of_spades.png",
        "assets/4_of_clubs.png",
        "assets/4_of_diamonds.png",
        "assets/4_of_hearts.png",
        "assets/4_of_spades.png",
        "assets/5_of_clubs.png",
        "assets/5_of_diamonds.png",
        "assets/5_of_hearts.png",
        "assets/5_of_spades.png",
        "assets/6_of_clubs.png",
        "assets/6_of_diamonds.png",
        "assets/6_of_hearts.png",
        "assets/6_of_spades.png",
        "assets/7_of_clubs.png",
        "assets/7_of_diamonds.png",
        "assets/7_of_hearts.png",
        "assets/7_of_spades.png",
        "assets/8_of_clubs.png",
        "assets/8_of_diamonds.png",
        "assets/8_of_hearts.png",
        "assets/8_of_spades.png",
        "assets/9_of_clubs.png",
        "assets/9_of_diamonds.png",
        "assets/9_of_hearts.png",
        "assets/9_of_spades.png",
        "assets/10_of_clubs.png",
        "assets/10_of_diamonds.png",
        "assets/10_of_hearts.png",
        "assets/10_of_spades.png",
        "assets/ace_of_clubs.png",
        "assets/ace_of_diamonds.png",
        "assets/ace_of_hearts.png",
        "assets/ace_of_spades.png",
        "assets/jack_of_clubs2.png",
        "assets/jack_of_diamonds2.png",
        "assets/jack_of_hearts2.png",
        "assets/jack_of_spades2.png",
        "assets/queen_of_clubs2.png",
        "assets/queen_of_diamonds2.png",
        "assets/queen_of_hearts2.png",
        "assets/queen_of_spades2.png",
        "assets/king_of_clubs2.png",
        "assets/king_of_diamonds2.png",
        "assets/king_of_hearts2.png",
        "assets/king_of_spades2.png",
    ];
    let mut cards: Vec<i32> = (0..52).collect(); //.collect() bascially means to take the range of 0 to 52 and plop them all into a vector with a unique ID on each one.
    let mut player_hand: Vec<i32> = vec![];
    let mut dealer_hand: Vec<i32> = vec![];
    let lbl_title = Label::new("Blackjack", 400.0, 150.0, 60);
    let mut lbl_money = Label::new("Cash: $100.00", 150.0, 100.0, 30);
    let mut lbl_dealer_score = Label::new("Dealer Hand (?)", 210.0, 185.0, 40);
    let mut lbl_player_score = Label::new("Player Hand ()", 210.0, 485.0, 40);
    let mut txt_bet = TextInput::new(200.0, 900.0, 300.0, 50.0, 25.0);
    let mut btn_hit = TextButton::new(350.0, 750.0, 150.0, 50.0, "Hit", GOLD, DARKBROWN, 30);
    let mut btn_stand = TextButton::new(550.0, 750.0, 150.0, 50.0, "Stand", GOLD, DARKBROWN, 30);
    let mut btn_exit = TextButton::new(750.0, 900.0, 150.0, 50.0, "Exit", RED, BLACK, 30);
    let mut btn_restart = TextButton::new(750.0, 750.0, 150.0, 50.0, "Restart", ORANGE, BLACK, 30);
    let mut btn_deal = TextButton::new(150.0, 750.0, 150.0, 50.0, "Deal", BLUE, DARKBLUE, 30);
    let mut account = 100.0;
    let mut game_start = false;
    let mut cards_revealed = 2;
    let mut dealer_cards_revealed = 1;
    let mut player_score = 0;
    let mut dealer_score = 0;
    let mut calc_score_player = true;
    let mut calc_score_dealer = false;
    let mut hit = false;
    let mut stand = false;
    txt_bet
        .set_prompt("Enter in bet amount...")
        .set_allowed_chars("1234567890.")
        .set_max_chars(26);
    btn_deal.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_exit.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_restart.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_stand.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_hit.with_text_color(BLACK).with_hover_text_color(WHITE);
    btn_hit.enabled = false;
    btn_stand.enabled = false;
    loop {
        clear_background(DARKGREEN);
        draw_rectangle(100.0, 50.0, 850.0, 800.0, GREEN);
        draw_rectangle(200.0, 200.0, 650.0, 200.0, DARKGREEN);
        draw_rectangle(200.0, 500.0, 650.0, 200.0, DARKGREEN);
        if btn_hit.click() {
            if cards_revealed < player_cards.len() {
                cards_revealed += 1;
            }
            hit = true;
            calc_score_player = true;
        }
        if btn_stand.click() {
            stand = true;
            calc_score_dealer = true;
        }
        if btn_exit.click() {
            break;
        }
        if btn_restart.click() {
            btn_deal.enabled = true;
            txt_bet.set_enabled(true);
            game_start = false;
            account = 100.0;
            lbl_money.set_text(format!("Cash: ${:.2}", account));
            player_hand.clear();
            dealer_hand.clear();
        }
        if btn_deal.click() {
            let bet = txt_bet.get_text().trim().parse().unwrap_or(0.0);
            if bet <= account && bet > 0.0 {
                let n = account - bet;
                lbl_money.set_text(format!("Cash: ${:.2}", n));
                txt_bet.set_enabled(false);
                account = n;
                btn_deal.enabled = false;
                cards.shuffle();
                player_hand.push(cards[0]);
                player_hand.push(cards[1]);
                dealer_hand.push(cards[2]);
                dealer_hand.push(cards[3]);
                btn_hit.enabled = true;
                btn_stand.enabled = true;
                game_start = true;
            } else {
            }
        }
        draw_grid(50.0, WHITE);
        lbl_money.draw();
        //if gamestart works fine but == true just makes it easier for me to read.
        if game_start == true {
            for i in 0..2 {
                player_cards[i].set_preload(tm.get_preload(card_names[cards[i] as usize]).unwrap());
                dealer_cards[0].set_preload(tm.get_preload(card_names[cards[i + 2] as usize]).unwrap());
                player_cards[i].draw();
                dealer_cards[i].draw();
            }
            if hit == true {
                for i in 2..cards_revealed {
                    player_cards[i].set_preload(tm.get_preload(card_names[player_hand[i] as usize]).unwrap());
                    player_cards[i].draw();
                }
            }
            if stand == true {
                btn_hit.enabled = false;
                while dealer_score < 17 {
                    dealer_cards_revealed+=1;
                    for i in 0..dealer_cards_revealed {
                        dealer_cards[i].set_preload(tm.get_preload(card_names[dealer_hand[i] as usize]).unwrap());
                        dealer_cards[i].draw();
                        card_value(dealer_hand[i]);
                    }
                }
            }
            if calc_score_player == true {
                player_score = 0;
                for i in 0..player_hand.len() {
                    player_score += card_value(player_hand[i]);
                }
                lbl_player_score.set_text(&format!("Player Hand ({})", player_score));
            }
            if calc_score_dealer == true {
                dealer_score = 0;
                for i in 0..dealer_hand.len() {
                    dealer_score += card_value(dealer_hand[i]);
                }
                lbl_dealer_score.set_text(&format!("Dealer Hand ({})", dealer_score));
            }
            calc_score_player = false;
            calc_score_dealer = false;
        }

        txt_bet.draw();
        lbl_title.draw();
        lbl_dealer_score.draw();
        lbl_player_score.draw();
        next_frame().await;
    }
}
//ask about using as in the code and .collect()
