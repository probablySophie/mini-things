pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod scene
{
	// Basically everything we make will need different scenes with diferent drawing & logic
}

mod io
{
	pub fn save() {}

	pub fn load() {}
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
