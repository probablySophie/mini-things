use std::io::stdout;

use crossterm::{cursor, execute, terminal};


pub struct Window
{
	
}
impl Drop for Window
{
	// So we get out of raw mode if we crash
	fn drop(&mut self) 
	{
	    terminal::disable_raw_mode()
			.expect("Could not disable raw mode");
	}
}
impl Window
{
	pub fn new() -> Window
	{
		terminal::enable_raw_mode()
			.expect("Could not enable raw mode");

		// make a window now, so that we'll leave raw mode if anything goes wrong
		let window = Window {};

		execute!(
				stdout(),
				terminal::EnterAlternateScreen,
				cursor::Hide,
			)
			.expect("Failed to enter alt screen");

		window // return windows
	}

	pub fn destroy(&mut self)
	{
		execute!(
				stdout(),
				terminal::LeaveAlternateScreen,
				cursor::Show
			)
			.expect("Failed to leave alt screen");

		terminal::disable_raw_mode()
			.expect("Failed to disable raw mode");
	}
}
impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}


pub fn clear_screen()
{
	execute!(
			stdout(),
			terminal::Clear(terminal::ClearType::All),
		)
		.expect("Failed to clear screen");
}


