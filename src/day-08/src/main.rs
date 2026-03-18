fn main() {
    let literal = "this is a string literal";
		println!("{literal}");

	let mut string = String::from("this is a 'String'");

	string.push_str(", with text appended using .push_str");

	println!("{string}");


	{
		let scope = String::from("inside a scope");
	
		println!("{scope}");
	} 		//scope over, rust calls a special function for us called "drop"
			//it allows the author of String to put the code to return the memory

	// println!("{scope}");
	// because the variable scope was dropped from memory after the scope  ended the compiler cannot find the variable

	string_function(String::from("this is a function"));	
}	

fn string_function(x: String) {

	println!("{x}");
}
