use crossterm::{event::{read, KeyCode, KeyEvent, MouseButton, MouseEvent}, execute, terminal};
use std::io::stdout;




struct Scene
{
	exit_key: KeyCode,
	should_exit: bool,
}
trait SceneFunctions
{
	fn logic();
	fn draw();
}

impl Scene
{
	// https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html
	fn key_pressed (&mut self, keycode: KeyCode)
	{
		match keycode
		{
			KeyCode::Char('q') => { },
			KeyCode::Char('w') => println!("You pressed W!"),
			_ => {},
		}	
	}

	fn mouse_pressed (&mut self, x: u16, y: u16, button: MouseButton)
	{
		println!("{} {} {:?}", x, y, button);
	}

	// TODO: have update types: on_event & constant
	fn run (&mut self)
	{
		// Our main loop
		loop
		{
			let event = read().unwrap();
		
			self.match_event(event);

			if self.should_exit
			{
				break;
			}
		}
	}

	fn match_event (&mut self, event: crossterm::event::Event)
	{
		match event
			{
				// https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html
				// https://docs.rs/crossterm/latest/crossterm/event/enum.KeyEventKind.html
				crossterm::event::Event::Key( KeyEvent {
					kind: crossterm::event::KeyEventKind::Press,
					code,
					.. }
				) => {
					if code == self.exit_key 
					{
						self.should_exit = true;
						return
					}
					self.key_pressed(code);
				},

				// https://docs.rs/crossterm/latest/crossterm/event/struct.MouseEvent.html
				crossterm::event::Event::Mouse( MouseEvent { 
					kind: crossterm::event::MouseEventKind::Down(button),
					row,
					column,
					.. } 
				) => {  
					self.mouse_pressed(column, row, button);
				},

				//crossterm::event::Event::FocusGained => todo!(),
				//crossterm::event::Event::FocusLost => todo!(),
				//crossterm::event::Event::Paste(_) => todo!(),
				//crossterm::event::Event::Resize(_, _) => todo!(),
			
				_ => {},
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

	
	let mut scene = Scene {
		should_exit: false,
		exit_key: KeyCode::Esc,
	};

	scene.run();
	
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
