use std::io::{self, Read};


pub struct SceneValues
{
	break_key_val: u8,
	should_exit_val: bool,
}
impl Default for SceneValues
{
    fn default() -> Self
	{
        Self
		{
			break_key_val: super::key::ESCAPE,
			should_exit_val: false,
		}
    }
}
impl SceneValues
{
	pub fn should_exit (&mut self) -> bool
	{
		self.should_exit_val
	}
	pub fn set_exit (&mut self, new_should_exit: bool)
	{
		self.should_exit_val = new_should_exit;
	}
	pub fn break_key(&mut self) -> u8
	{
		self.break_key_val
	}
	pub fn set_break_key(&mut self, key: u8)
	{
		self.break_key_val = key;
	}
}


pub enum UpdateType {
	OnKeypress,
	Constant,
}

pub trait TerminalScene: mini_core::Scene
{	
	fn break_key(&mut self) -> u8;
	fn set_break_key(&mut self, key: u8);

	fn run (&mut self, update_type: UpdateType)
	{
		match update_type {
			UpdateType::OnKeypress => self.loop_on_keypress(),
			UpdateType::Constant => todo!(),
		}
	}

	fn loop_on_keypress (&mut self)
	{
		let mut buf = [0; 1]; // our key pressed buffer

		// draw the first time manually so we don't have to wait for a keypress event
		self.draw();

		while io::stdin().read(&mut buf).expect("Failed to read line") == 1
		{
			// quit if the user pressed the quit key
			if buf[0] == self.break_key() { break }

			// should we exit?
			if self.should_exit() { break }

			self.key_pressed(buf[0]);
			self.logic();
			self.draw();

			// check again so we don't have to wait for the next keypress
			if self.should_exit() { break }
		}
	}
}

