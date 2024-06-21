use std::net::TcpListener;

// I guess we're following this guy: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() 
{
	println!("Hello, world!");

	let listener: TcpListener;

	if let Ok(new_listener) = TcpListener::bind("127.0.0.1:7878")
	{
		listener = new_listener;
	}
	else 
	{
		println!("Unable to bind to port!");
		return;
	}



	for stream in listener.incoming()
	{
		let stream = stream.unwrap();

		println!("That's a connection!");
	}
}

