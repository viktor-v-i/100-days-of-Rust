fn main() {
 	let greeting =  make_greeting("greetings");
	println!("{greeting}");

	let first = count_words("first word");
	println!("{first}");	

	let mut example = String::from("shout"); 
	shout(&mut example);
	println!("{example}");
}

fn make_greeting(s: &str) -> String {
	let mut hello = String::from("greetings");
	hello.push_str(",");
	hello.push_str(s);
	hello
} 

fn count_words(s: &str) -> usize {
	let bytes = s.as_bytes();
	let mut count = 0;
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
		count += 1;
		}
		
	}
	count + 1
}

fn shout(string: &mut String) {
	string.push_str("!!!");
}
