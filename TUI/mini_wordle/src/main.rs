use scenes::game::MainGame;
use TUI::{scene::{TerminalScene, UpdateType}, Window};

mod scenes;
mod board;

fn main ()
{
	let mut window = Window::new();

	// TODO: main_game
	
	// TODO: game_over

	let mut main_game = MainGame::default();

	main_game.run(UpdateType::OnKeypress);
	// game_over.run_scene(UpdateType::OnKeypress)
	
	window.destroy();
}

