pub fn add(left: usize, right: usize) -> usize {
    left + right
}


mod board
{
	// game board
}


mod stats
{
	// Game stats & file formats (TOML)
}

/* ~~ GAME SCENES ~~ */

mod playing
{
	//
}

mod game_over
{
	//
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
