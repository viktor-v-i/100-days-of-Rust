fn main() {

	let immutable = 10;
	
	println!("The value of the variable 'immutable' is ({immutable}), it cannot be assinged a new value");
	println!("In order to change it's value we need to add 'mut' after 'let'");
	
	let mut mutable = 11;
	
	println! ("the value of the mutable variable ({mutable}) can be changed");

	mutable = 12;
	println! ("take a look: {mutable}");

	println!("shadowing allows me to change an immuable value within a defined scope");
	
	{
		let immutable = 13;
		println!("the value of immutable ({immutable}) is different within this inner scope");
	}
	
	let immutable = 5;
	println!("by writing 'let immutable = 5' i can rebind the immutable variable ({immutable}) with a new value!");
	println!("shadowing without using an inner scope");
	
	println!("constants are declared 'const' they are always immutable");
	println!("they differ from variables in that they are assinged to a constant expression rather than value");
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Example taken from the book.
	println!("{THREE_HOURS_IN_SECONDS}");
}
