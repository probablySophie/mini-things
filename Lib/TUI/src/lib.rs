use std::io::stdout;

use crossterm::{cursor, execute, terminal::{self, Clear}};

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

	pub fn destroy()
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
			Clear(terminal::ClearType::All),
		)
		.expect("Failed to clear screen");
}


mod print
{
    use std::io::stdout;

    use crossterm::{cursor, execute, style::Print};

	// TODO: Swap out String for crossterm's fancy string
	pub fn string(string: String, x: u16, y: u16)
	{
		execute!(
				stdout(),
				cursor::MoveTo(x,y),
				Print(string),
			)
			.expect("Failed to print string");
	}

	pub fn vec(vec: Vec<String>, x: u16, y: u16)
	{
		for (i, line) in vec.iter().enumerate()
		{
			string(line.clone(), x, y + (i as u16));
		}
	}
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
