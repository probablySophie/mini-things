
use std::io::stdout;

use crossterm::{cursor, execute, style::{Print, StyledContent}};

pub fn string(string: String, x: u16, y: u16)
{
	execute!(
			stdout(),
			cursor::MoveTo(x,y),
			Print(string),
		)
		.expect("Failed to print string");
}

pub fn styled(string: StyledContent<String>, x: u16, y: u16)
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
