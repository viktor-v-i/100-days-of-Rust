const SEPERATOR: &str = "---";
struct Book {
	title: String,
	author: String,
	pages: u32,
	read: bool,
	}
	
	
fn main() {
	let mut rust = Book {
		title: String::from("Rust"),
		author: String::from("This Author"),
		pages: 400,
		read: false
	};
	println!("title: {}",  rust.title,); 
	println!("author: {}",  rust.author,);		
	println!("pages: {}",  rust.pages,);
	println!("read: {}",  rust.read,);

	println!("{SEPERATOR}");
	println!("before: {}", rust.read);
	
	rust.read = true;
	println!("after: {}", rust.read);
	println!("{SEPERATOR}");

	let title = build_book(String::from("build"), String::from("author"));
	println!("build book: {}", title.title );
}

fn build_book(title: String, author: String) -> Book { 
	Book {
	title: title,
	author: author,
	pages: 460,
	read: false,
	}
}
