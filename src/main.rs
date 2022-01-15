#![allow(dead_code)]

mod game;
mod map;
mod entity;
mod player;
mod screen;
mod widget;
mod macros;
mod layout;
mod misc;
mod input;
mod gameui;

use game::Game;

fn main()
{
    let mut game = Game::new();
    game.splash_screen();
    game.start_menu();
    game.start();
}
