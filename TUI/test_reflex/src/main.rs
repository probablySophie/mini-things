use crossterm::{event::{read, KeyCode, MouseEventKind}, execute, terminal};
use std::io::{self, stdout};

struct Scene
{
	should_exit: bool,
}

impl Scene
{
	fn key_pressed (&mut self, keycode: KeyCode)
	{
		match keycode
		{
			KeyCode::Char('q') => {self.should_exit = true},
			KeyCode::Char('w') => println!("You pressed W!"),
			_ => {},
		}
	}

	fn run (&mut self)
	{
		loop
		{
			match read().unwrap()
			{
				crossterm::event::Event::Key(keyevent) => { self.key_pressed(keyevent.code) },

				crossterm::event::Event::Mouse(kind: MouseEventKind::Down, mouseevent) => {},

				//crossterm::event::Event::FocusGained => todo!(),
				//crossterm::event::Event::FocusLost => todo!(),
				//crossterm::event::Event::Paste(_) => todo!(),
				//crossterm::event::Event::Resize(_, _) => todo!(),
			
				_ => {},
			}

			if self.should_exit
			{
				break;
			}
		}
	}
}


fn main() 
{
	terminal::enable_raw_mode()
			.expect("Could not enable raw mode");

	execute!(stdout(),	
		crossterm::event::EnableMouseCapture,
	).expect("");

			

	
	execute!(stdout(),
		crossterm::event::DisableMouseCapture,
	).expect("");
	
	terminal::disable_raw_mode()
			.expect("Could not disable raw mode");
}

// Event: pressed enter
// Key(KeyEvent { code: Enter, modifiers: KeyModifiers(0x0), kind: Press, state: KeyEventState(0x0) })
//
// Resite(u16, u16)
//
// Key(KeyEvent { code: Char('c'), modifiers: KeyModifiers(0x0), kind: Press, state: KeyEventState(0x0) })
