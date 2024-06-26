
// The board is made up of:
// * The rows of tiles
// * The keyboard display

use super::{letter_state::LetterState, words};

pub const BOARD_WIDTH:  usize = 5;
pub const BOARD_HEIGHT: usize = 6;

pub type BoardTiles = [
			[(LetterState, char); BOARD_WIDTH] // the row
			; BOARD_HEIGHT // the column
		];

pub type BoardKeyboard = [(LetterState, char); 26];



pub struct Board 
{
	pub tiles: BoardTiles,

	pub keyboard: BoardKeyboard,
	
	pub active_row: usize,
	pub active_col: usize,

	word: String,
	words: Vec<String>,
}
impl Default for Board
{
	fn default() -> Self
	{
		let mut keyboard = [(LetterState::default(), ' '); 26];

		for (i, char) in "qwertyuiopasdfghjklzxcvbnm".chars().enumerate()
		{
			keyboard[i].1 = char;
		}

		let loaded_words: Vec<String> = words::load_words().unwrap();

		Board
		{
			tiles: [ [(LetterState::default(), ' '); BOARD_WIDTH]; BOARD_HEIGHT ],
			keyboard,
			active_row: 0,
			active_col: 0,
			word: words::new_word(&loaded_words),
			words: loaded_words,
		}
	}
}

impl Board
{
	pub fn make_guess(&mut self) -> Result<bool, ()>
	{
		// if the current row isn't full, then we can't make a guess
		if let Ok(full) = self.row_full()
		{
			// if the row ISN'T full, we can't guess
			if ! full
			{
				return Err(())
			}
		}
		else // else we weren't able to make a guess
		{
			return Err(())
		}

		// if we've gotten here, the row IS full & the board ISN'T full
		
		let guess_word = self.current_guess_word();

		// is the current word valid?
		if ! words::is_valid(guess_word.clone(), &self.words)
		{
			return Err(()) // the current word isn't a valid word
		}

		// TODO: Update the tile & keyboard LetterStates

		let result = self.mark_word(guess_word);

		self.active_row += 1; // update where we're looking
		self.active_col = 0;

		// did we win?
		if result
		{
			return Ok(true) // yay!
		}

		Ok(false) // if we get here, the guess was wrong
	}

	pub fn new_letter(&mut self, letter: char) -> Result<(), ()>
	{
		// if the board is full
		if self.board_full()
		{
			return Err(()) // we can't add any new letters
		}

		// if the row is full
		if self.active_col >= BOARD_WIDTH
		{
			return Err(()) // we can't add any new letters
		}

		// if we've gotten here, we are allowed to add a letter

		self.tiles[self.active_row][self.active_col].1 = letter;
		
		self.active_col += 1;

		Ok(())
	}

	pub fn remove_letter(&mut self) -> Result<(), ()>
	{
		// If the board is full, there's nothing we can do
		if self.board_full()
		{
			return Err(())
		}

		// is the row empty?
		if self.active_col == 0
		{
			return Err(()) // then we aren't able to delete any letters
		}

		// empty the previous column
		self.tiles[self.active_row][self.active_col - 1].1 = ' ';

		self.active_col -= 1;
				
		Ok(()) // we're done :)
	}


	fn board_full(&mut self) -> bool
	{
		if self.active_row >= BOARD_HEIGHT
		{
			return true
		}
		false
	}

	fn row_full(&mut self) -> Result<bool, ()>
	{
		if self.board_full()
		{
			return Err(()) // if the board is full, the row is full too
		}

		// is the row full?
		if self.active_col >= BOARD_WIDTH
		{
			return Ok(true) // yes
		}
		Ok(false) // no
	}

	fn current_guess_word(&mut self) -> String
	{
		let mut guess_word: String = "".to_owned();

		for tile in self.tiles[self.active_row]
		{
			guess_word += &tile.1.to_string();
		}

		guess_word
	}


	fn mark_word (&mut self, guess: String) -> bool
	{
		let mut score: [LetterState; BOARD_WIDTH] = [LetterState::Wrong; BOARD_WIDTH];
		let mut word: Vec<char> = self.word.chars().collect();


		// Is this an efficient way of doing this?  No.
		// Does it work?  Maybe?

		// check for perfects
		for (i, letter) in guess.chars().enumerate()
		{
			for (x, word_letter) in word.clone().iter().enumerate()
			{
				// if the letter is in the word string AND they're in the same location
				if (&letter == word_letter) && (i == x)
				{
					score[i] = LetterState::Perfect;

					word[x] = ' ';
				}
			}
		}

		// go back for almosts
		for (i, letter) in guess.chars().enumerate()
		{
			for (x, word_letter) in word.clone().iter().enumerate()
			{
				match score[i]
				{
					LetterState::Perfect => {},	

					_ => {
						if &letter == word_letter
						{
							score[i] = LetterState::Almost;

							word[x] = ' ';
						}
					}
				}
			}
		}

		// THEN update our keyboard

		for (key_num, key) in self.keyboard.clone().iter().enumerate()
		{
			for (i, letter) in guess.chars().enumerate()
			{
				if key.1 == letter
				{
					// only update if we have a positive information change
					match (score[i], key.0)
					{
						(_, LetterState::Unused)  |  // anything is an improvement over unused
						(LetterState::Perfect, _) |  // perfect is always best
						(LetterState::Almost, LetterState::Wrong) // almost > wrong
						=> {self.keyboard[key_num].0 = score[i]},

						_ => {}, // otherwise, do nothing	
					}
				}
			}
		}

		// Then update the board

		for (i, score) in score.iter().enumerate()
		{
			self.tiles[self.active_row][i].0 = *score;
		}

		if guess == self.word
		{
			return true; // we win
		}
		false // we don't win :(
	}
}
