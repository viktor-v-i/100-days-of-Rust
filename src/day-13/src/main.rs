fn main() {
   let string = String::from("Five first characters");
	
	let five = &string[0..5];

	println!("{five}");

	let three = &string[3..];
	
	println!("{three}");

	let whole = &string[..];

	println!("{whole}");

	let inital = first_word("no spaces");
	println!("{inital}");

	let second = first_word("nospaces");
	println!("{second}");
	
	let third = first_word("slices_of slices");
	println!("{third}");
	
	let array = [1, 2, 3, 4, 5];

	let a_slice = &array[1..4];
	
	for element in a_slice  {
		println!("{element}");
	}
}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	
	&s[..]
}

