use TUI::scene::TerminalScene;



pub struct MainGame
{
	scene_values: TUI::scene::SceneValues,
	game_board: wordle::game::board::Board,
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

const KEY_ENTER: u8 = 13;
const KEY_DELETE: u8 = 127;

impl mini_core::Scene for MainGame
{
	fn logic (&mut self)
	{
		//
	}

	fn draw (&mut self)
	{
		crate::board::draw_tiles(self.game_board.tiles);
			
		crate::board::draw_keyboard(self.game_board.keyboard);
	}

	fn key_pressed(&mut self, key: u8) {
    
		match key
		{
			KEY_ENTER => {_ = self.game_board.make_guess()},

			KEY_DELETE => {_ = self.game_board.remove_letter()},

			_ => {
				let letter = char::from(key);
				if letter.is_alphabetic()
				{
					_ = self.game_board.new_letter(letter);
				}
			},
		};
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
			game_board: wordle::game::board::Board::default(),
		}
	}
}
