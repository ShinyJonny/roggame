#![allow(dead_code)]

mod game;
mod map;
mod entity;
mod player;
mod screen;

fn main()
{
    let mut g = game::Game::new();
    g.init_maps(vec!["map.dat", "map.dat"]);
}
