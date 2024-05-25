use TUI::{scene::{TerminalScene, UpdateType}, Window};



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

struct MainGame
{
	scene_values: TUI::scene::SceneValues,
}

impl TerminalScene for MainGame
{
    fn break_key(&mut self) -> u8 {
        self.scene_values.break_key()
    }

    fn set_break_key(&mut self, key: u8) {
        self.scene_values.set_break_key(key);
    }
	//
}

impl mini_core::Scene for MainGame
{
	fn logic (&mut self)
	{
		TUI::print::string("logic".to_owned(), 1, 4);
	}

	fn draw (&mut self)
	{
		TUI::print::string("draw".to_owned(), 10, 10);
	}

	fn key_pressed(&mut self, key: u8) {
        // TODO:
    }


	fn should_exit (&mut self) -> bool
	{
		self.scene_values.should_exit()
	}

	fn set_exit (&mut self, new_value: bool)
	{
		self.scene_values.set_exit(new_value);
	}
}


impl Default for MainGame
{
	fn default() -> Self
	{
		Self
		{
			scene_values: TUI::scene::SceneValues::default(),
		}
	}
}
