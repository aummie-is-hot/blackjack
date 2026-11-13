/*
By: <Your Name Here>
Date: 2026-11-09
Program Details: <Program Description Here>

let btn_5_chip = ImageButton::from_preload(
        100.0,
        200.0,
        200.0,
        60.0,
        tm.get_preload("assets/button.png").unwrap(),
        tm.get_preload("assets/button_hover.png").unwrap(),
    );
     let btn_25_chip = ImageButton::from_preload(
        100.0,
        200.0,
        200.0,
        60.0,
        tm.get_preload("assets/button.png").unwrap(),
        tm.get_preload("assets/button_hover.png").unwrap(),
    );
     let btn_100_chip = ImageButton::from_preload(
        100.0,
        200.0,
        200.0,
        60.0,
        tm.get_preload("assets/button.png").unwrap(),
        tm.get_preload("assets/button_hover.png").unwrap(),
    );
*/

mod ui;
mod utils;
use crate::ui::grid::draw_grid;
use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
 use crate::ui::image_button::ImageButton;
     use crate::ui::label::Label;
 use crate::ui::text_button::TextButton;
   use crate::ui::text_input::TextInput;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "blackjack".to_string(),
        window_width: 1024,
        window_height: 768,
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
     let mut money = 1000;
     let mut lbl_money = Label::new("Money: ", 50.0, 580.0, 40);
        lbl_money.with_colors(WHITE, Some(DARKGREEN));
    let mut btn_stand = TextButton::new(
        400.0,
        670.0,
        200.0,
        60.0,
        "Stand",
        WHITE,
        GREEN,
        30
    );
    let mut btn_hit = TextButton::new(
        650.0,
        670.0,
        200.0,
        60.0,
        "Hit",
        WHITE,
        BLUE,
        30
    );
    let mut btn_deal = TextButton::new(
        150.0,
        670.0,
        200.0,
        60.0,
        "Deal",

        WHITE,
        PURPLE,
        30
    );
      let mut btn_exit = TextButton::new(
        850.0,
        50.0,
        200.0,
        60.0,
        "Exit",

        WHITE,
        RED,
        30
    );
     let mut txt_input = TextInput::new(400.0, 550.0, 200.0, 40.0, 25.0);
     txt_input.set_allowed_chars("0123456789");
     txt_input.set_prompt("Input bet amount");
    btn_stand.with_text_color(BLACK); // Sets the normal text color
    btn_stand.with_hover_text_color(WHITE);
    btn_hit.with_text_color(BLACK); // Sets the normal text color
    btn_hit.with_hover_text_color(WHITE);
    btn_deal.with_text_color(BLACK); // Sets the normal text color
    btn_deal.with_hover_text_color(WHITE);
    btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    loop {
        clear_background(DARKGREEN);

        
        draw_rectangle(0.0, 630.0, 1200.0, 300.0, DARKGRAY);
        
        if btn_deal.click(){

        }
         if btn_hit.click(){

        }
         if btn_stand.click(){

        }
         if btn_exit.click(){

        }
         txt_input.draw();
         lbl_money.set_text(format!("Money: {}", money));
         lbl_money.draw();
        next_frame().await;
    }
}
