
// Take a string, split it by a give char
fn split_to_strings(text: String, split_at: char) -> Vec<String>
{
	return text.split(split_at)
		.map(|s| s.to_string())
		.collect::<Vec<String>>()
}

// Parse a string into a Vec<String> of headings & a Vec<Vec<String>> of values
pub fn parse(csv: String) -> (Vec<String>, Vec<Vec<String>>)
{
	let mut values: Vec<Vec<String>> = Vec::new();

	let csv_lines = split_to_strings(csv, '\n');	

	let headings = split_to_strings(csv_lines[0].clone(), ',');

	// skip line 1
	for line in &csv_lines[1..]
	{
		// ignore empty lines
		if line.is_empty()
		{
			values.push(
				split_to_strings(line.to_owned(), ',')
			);
		}
	}

	(headings, values) //return (headings, values);
}
