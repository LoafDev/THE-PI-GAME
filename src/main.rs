#![windows_subsystem = "windows"]

mod icon_data;
use icon_data::*;

use macroquad::prelude::{
    RED,
    Conf,
    load_ttf_font,
    TextParams,
    prevent_quit,
    next_frame,
    get_frame_time,
};

use macroquad::miniquad::conf::Icon;
use macroquad::time::get_time;
use pi_tester::{
    update,
    W,
    H,
    draw,
    Game
};

fn win_conf() -> Conf {
    let ic = Icon {
        small: ICON_SMALL,
        medium: ICON_MEDIUM,
        big: ICON_BIG
    };

    Conf {
        window_height: H,
        window_width: W,
        window_title: "THE PI GAME".to_string(),
        window_resizable: false,
        fullscreen: false,
        icon: Some(ic),
        ..Default::default()
    }
}

#[macroquad::main(win_conf)]
async fn main() {
    let text_font = load_ttf_font("hi.ttf").await.unwrap();
    let text_font2 = load_ttf_font("perfect.ttf").await.unwrap();

    let param1 = TextParams {
        font_size: 50,
        font: Some(&text_font),
        color: RED,
        ..Default::default()
    };

    let param2 = TextParams {
        font_size: 30,
        font: Some(&text_font2),
        color: RED,
        ..Default::default()
    };

    let mut game = Game::default();
    prevent_quit();

    loop {
        if game.quit_ok {break;}

        let delta = get_frame_time();
        let init_time = get_time() as f32;

        update(&mut game, delta);
        draw(&mut game, &param1, &param2, init_time);

        next_frame().await;
    }
}