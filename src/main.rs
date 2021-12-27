#![allow(dead_code)]

mod game;
mod map;
mod entity;
mod player;
mod screen;
mod widget;
mod terminal;
mod macros;

use game::Game;

fn main()
{
    let mut game = Game::new();

    game.load_maps(vec!["map.dat"]);
    game.init_player();
    game.start();
}
