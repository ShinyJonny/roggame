#![allow(dead_code)]
mod game;
mod map;
mod player;
mod screen;
mod widget;
mod macros;
mod layout;
mod misc;
mod input;
mod gameui;
mod style;

use game::Game;

fn main()
{
    let mut game = Game::new();
    game.run();
}
