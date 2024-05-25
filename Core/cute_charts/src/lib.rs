#[allow(dead_code)]

pub mod charts;



// The old cute_charts repo is at: https://github.com/probablySophie/cute_charts





/* Straight from the main.rs of cute charts
* We'll want to use core::csv & core::file

fn main() 
{
	//let args: Vec<String> = env::args().collect(); // get the program args
	
	//TODO: Iterate through the args

	//let csv_string: String = args[1].clone();
	
	//TODO: Allow user specified input
	let csv_string: String = fs::read_to_string("sample.csv").unwrap().parse().unwrap();

	//TODO: Allow the user to specify an output file

	let (headings, values) = csv::parse(csv_string);

	// input file
	// output file

	//TODO: Allow different types of chart

	

	charts::bar::horizontal(headings, values);
}

*/


pub fn largest_value(values: Vec<i32>) -> i32
{
	let mut largest = values[0];

	for value in values.iter()
	{
		if value > &largest
		{
			largest = *value;
		}
	}

	return largest;
}

// Assuming the values are in form (name, value)
pub fn longest_value_name(headings: Vec<String>, values: Vec<Vec<String>>) -> u32
{
	let mut longest = headings[0].len();

	for value_pair in values.iter()
	{
		let length = value_pair[0].len();
			
		if length > longest
		{
			longest = length;
		}
	}

	return longest as u32;
}
