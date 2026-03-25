const SEPARATOR: &str = "---";

fn main() {

 	println!("{SEPARATOR}");

	let mut greeting =  make_greeting("Name");
	println!("{greeting}");

	println!("{SEPARATOR}");

	let first = count_words(&greeting);
	println!("{first}");

	println!("{SEPARATOR}");

	let _shout = shout(&mut greeting);
	println!("{greeting}");
	
	println!("{SEPARATOR}");

	let mut clone = greeting.clone();
	clone.push_str(" Cloned");
	println!("{clone}");

	println!("{SEPARATOR}");

	let slice = first_word("First word");
	println!("{slice}");
}

fn make_greeting(s: &str) -> String {
	let mut hello = String::from("Greeting");
	hello.push_str(", ");
	hello.push_str(s);
	hello
} 

fn count_words(s: &str) -> usize {
	let bytes = s.as_bytes();
	let mut count = 0;
	
	for (_i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
		count += 1;
		}
		
	}
	count + 1
}

fn shout(string: &mut String) {
	string.push_str("!!!");
}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i]
		}
	}
	
	&s[..]

}
