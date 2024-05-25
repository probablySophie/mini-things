
pub trait Scene
{
	fn logic (&mut self);
	fn draw (&mut self);

	fn should_exit (&mut self) -> bool;
	fn set_exit(&mut self, should_exit: bool);

	fn key_pressed(&mut self, key: u8);
}
/*
pub struct Scene
{
		// TODO: Contain some form of #LOGIC 
		// TODO: Contain some form of #DRAWING
		//
		// Those should probably both be some form of extends thingy?
}
*/

// This module will want to define what a scene is

// How scenes are run & drawn will need to be up to the GUI module


