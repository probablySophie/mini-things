use rand::Rng;


// Read in our words file
const WORDS: &str = include_str!("../words.txt");

pub fn load_words () -> Result<Vec<String>, ()>
{
	Ok (
		WORDS.split('\n')
			.map(|s| s.to_string())
			.collect()
	)
}


pub fn is_valid(word: String, words: &Vec<String>) -> bool
{
	for valid_word in words
	{
		if word == *valid_word
		{
			return true;
		}
	}
	false
}


pub fn new_word(words: &Vec<String>) -> String
{
	let mut rng = rand::thread_rng();

	let random_number: usize = rng.gen_range(0..words.len());

	words[random_number].clone()
}

