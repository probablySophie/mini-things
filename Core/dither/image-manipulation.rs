

// This guy was doing their own thing beforehand, but they can 100% be turned into a library!

use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel};


fn main() {

	let mut img = load("image.jpg");

	println!("Greyscale time!");
	img = img.grayscale();

	let (width, height) = (30, 60);

	println!("Resizing!");
	img = img.resize(width, height, image::imageops::FilterType::Gaussian);

	let (real_width, real_height) = (img.width(), img.height());
	
	// save the luma value of each pixel
	let mut processed = img.pixels().map(|pixel|
		{
			pixel.2[0]
		});

	for y in 0..real_height/2
	{
		for x in 0..real_width
		{
			let n: usize = ( (real_width*y) + x ) as usize;

			if processed.nth(n).unwrap() > 150
			{
				print!("█");
			}
			else if processed.nth(n).unwrap() > 100
			{
			    print!("▞");
			}
			else 
			{
				print!(" ");
			}
		}
		println!();
	}

	println!("{:?}", img.pixels().nth(1).unwrap().2.to_luma());



	save(img, "image2.jpg");
}

fn load(file_location: &str) -> DynamicImage
{
	println!("Reading: {}", file_location);

	ImageReader::open(file_location)
		.expect("Unable to open image")
		.decode()
		.expect("Unable to decode image")
}

fn save(img: DynamicImage, name: &str)
{
	println!("Saving as {}", name);

	img.save(name)
		.expect("Unable to save image");
}
