fn main() {
	print_string(String::from("string"));

	let x = String::from("second string");

	print_string(x);

	// println!("{x}"); <- causes and error because the ownership was moved, fn cannot use x anymore because String doesn't implement the copy trait
	// String gets stored in the heap because it's size can shrink and grow. 

	print_number(200);

	let y = 300;

	print_number(y);

	println!("{y}"); // it works because i32 are scalar and implements the copy trait, the data is fixed at compile time so it gets sent to the stack.

	let s2 = give_ownership();

	println!("{s2}");
	
	let s3 = String::from("returned");
	let s4 = take_and_give_back(s3);

	println!("{s4}");

	let (power, now) = calculate_length(String::from("p"));	
	println!("{power} {now}");
}

fn print_string(s: String) {
	println!("{s}");

} 

fn print_number(s1: i32) {
	println!("{s1}");
	
}

fn give_ownership() -> String { // the function transfers ownership to the caller, main receives that owenership by storing the return value in s2 
	let string_s2 = String::from("string_s2");

	string_s2	

}

fn take_and_give_back (s5: String) -> String {
	s5
}

fn calculate_length (s6: String) -> (String, usize) { // String and usize descriibe what the function returns when it gets called
	let a = s6.len();

	(s6, a)
}

