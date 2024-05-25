use super::letter_state::LetterState;




pub fn load_words ()
{
	// TODO: load_words
}


pub fn is_valid(word: String)
{
	// TODO: is_valid?
}


pub fn new_word() -> String
{
	// TODO: new_word
	"hi".to_string()
}


pub fn check_word(guess: String, word: String) -> [LetterState; 5]
{
	/*
	let local_word = word.clone();

	for letter in guess.clone().chars()
	{
		//
	}
	*/

	[LetterState::Perfect, LetterState::Perfect, LetterState::Perfect, LetterState::Perfect, LetterState::Perfect]
}
