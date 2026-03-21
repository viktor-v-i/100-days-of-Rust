fn main() {

	//req 1
	let s1 = String::from("string");

	let len = calculate_length(&s1); //creating a reference the function borrows

	println!("the length of {s1} is {len}.");

	// req 2
	let s2 = String::from("string_2");
	
	print_string(&s2);	// creating a reference the function borrows 
	
	print_string(&s2);	// creating a reference the function borrows
	println!("{s2}");

	//req 3
	let string = &s1; // referencing
	let string_2 = &s1; // referencing
	println!("{string} and {string_2}");

	// modify(&s2);
}

fn calculate_length(s: &String) -> usize {
	s.len()
} 

fn print_string(d: &String) {
	 println!("{d}")
}

// fn modify(f: &String) {
//	f.push_str(", push");
//} when you borrow with ampersand you get an immutable reference,
//	run with &mut in order to get a mutable reference
