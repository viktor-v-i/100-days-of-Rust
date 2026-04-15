struct Book {
	title: String,
	author: String,
	pages: u32,
	read: bool,
	}


fn main() {
	let book = build_book(String::from("book"), String::from("author")); 
	println!("title: {}", book.title);
	println!("author: {}", book.author);

	let book2 = Book {
	title: String::from("book2"),
	..book
	};
	println!("title: {}", book2.title); 
	println!("title_first: {}", book.title);
	// println!("pages: {}", book.author); - fails because it transfers owenership from book into book2. Book lost access, book2 got it's author from book. While title works because book2 provided it's own title
	println!("pages: {}", book.pages); //because u32 values are known they're put on the stack and can be copied.
}


fn build_book(title: String, author: String) -> Book {
	Book {
	title,
	author,
	pages: 400,
	read: true,
	}
		

}

