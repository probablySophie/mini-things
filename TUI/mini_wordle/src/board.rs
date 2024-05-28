use wordle::game::letter_state::LetterState;
use crossterm::style::{StyledContent, Stylize};

pub fn draw_tiles (tiles: wordle::game::board::BoardTiles)
{
	for (y, row) in tiles.iter().enumerate()
	{
		for (x, tile) in row.iter().enumerate()
		{
			let tile_string = " ".to_string() + &(tile.1.to_string() + &" ");

			TUI::print::styled( 
				style_me(tile_string, tile.0),
				(7 + x*3) as u16,
				y as u16);
		}
	}
}

pub fn draw_keyboard (keyboard: wordle::game::board::BoardKeyboard)
{
	// new line @ 9 & 18
	
	let mut y = wordle::game::board::BOARD_HEIGHT + 1;
	let mut x = 0;

	for (i, key) in keyboard.iter().enumerate()
	{
		let key_string = " ".to_string() + &(key.1.to_string() + &" ");

		TUI::print::styled(style_me(key_string, key.0), x, y as u16);

		match i 
		{
			9  => {y += 1; x = 1},
			18 => {y += 1; x = 4}, 
			_ => {x += 3},
		};
	}
}

fn style_me(string: String, value: LetterState) -> StyledContent<String>
{
	match value
	{
		LetterState::Perfect => { string.black().on_green()  },
		LetterState::Almost  => { string.black().on_yellow() },
		LetterState::Wrong   => { string.black().on_grey()   },
		LetterState::Unused  => { string.reset().bold()      },
	}
}
