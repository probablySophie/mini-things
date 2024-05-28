#![allow(non_snake_case)]

mod window;			pub use window::*;
pub mod print;
pub mod scene;






pub mod key
{
	pub const ESCAPE: u8 = 27;
	pub const DELETE: u8 = 127;
	pub const ENTER:  u8 = 13;
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
