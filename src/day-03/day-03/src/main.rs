fn main() {

	let signed: i32 = -5;
	println!("a signed integer: {signed}");

	let unsigned: u32 = 5;
	println!("an unsigend integer: {unsigned}");

	let floating: f32 = 3.0;
	println!("a floating-point number: {floating} uses decimals");

	let array = [5, 4, 3, 2, 1];
	
	let first = array[0];
	let last = 	array[4];
	println!("a simple array, here's the first value: {first},");
	println!("here's the last: {last}");
	
	let array_block = [5; 5];	
	
	let f = array_block[2];
	let g = array_block[0];	
	println!("{f}");	
	println!("{g}");
	println!("as you can see above-");
	println!("[value; count] allows you to create an array with the same values");
	
	let bool: bool = true;
	println!("a boolean: {bool}");

	let tup: (i32, u32, f32) = (-10, 0x5, 6.4);
	
	let (a, b, c) = tup;
	println!("this is value the first value of the destructured tuple: {a}");										

	let middle_tuple = tup.1;
	let last_tuple = tup.2;
	println!("these are the other values respectivly: {middle_tuple} and {last_tuple} ");

	let special_character: char = 'Δ';
	println!("you can use the char keyword for special characters: {special_character}");
}

