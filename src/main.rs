mod minesweeper;
use minesweeper::minesweeper_game::Game;

fn main()
{
    let mut game = Game::new();
    game.game_loop();
} // fn main
