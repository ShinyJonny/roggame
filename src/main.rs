#![allow(dead_code)]

mod game;
mod map;
mod entity;
mod player;
mod screen;

use game::Game;

fn main()
{
    let mut game = Game::init();

    game.load_maps(vec!["map.dat"]);
    game.init_player();
    game.start();
}
