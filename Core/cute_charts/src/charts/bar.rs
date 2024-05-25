use crate::charts::shared;

pub fn horizontal(headings: Vec<String>, values: Vec<Vec<String>>)
{
	//TODO: We need
			// Longest Heading/Value name
			// Largest Value

	//TODO: Allow a scale specifier so that everything doesn't become MASSIVE

	let name_length: usize = shared::longest_value_name(headings.clone(), values.clone()) as usize;

	let int_values = values.iter().
		map(|v| v[1].parse::<i32>().unwrap())
		.collect::<Vec<i32>>();

	let max_value = shared::largest_value(int_values.clone());

	println!("{}{}  {}", 
		headings[0], 
		" ".repeat(name_length-headings[0].len()), 
		headings[1], 
		);

	println!("");

	for i in 0..values.len()
	{
		let name = values[i][0].clone();

		println!("{}  {}{}  {}", 
			name,
			" ".repeat(name_length - name.len()),
			int_values[i],
			horizontal_bar(int_values[i] as usize, max_value as usize)
			);
	}
}

fn horizontal_bar(value: usize, space: usize) -> String
{
	return "█".repeat(value) + &("░".repeat(space-value));
}



