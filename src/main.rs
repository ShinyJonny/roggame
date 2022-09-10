mod game;
mod map;
mod player;
mod input;
mod gameui;

use game::Game;

fn main()
{
    let mut game = Game::new();
    game.run();
}
