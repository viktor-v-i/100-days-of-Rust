fn main() {
    print_something(10);

	let y = {
		let x = 3;
		x + 1
	};

	println!("an expression using a scope bock: {y}");

	let x = return_value(6.7);
	println!("{x} prints");
	
	two_parameters('#', 0x754);

	let x = return_early('&');	
	println!("a return keyword in use {x}");

	let x = i_32(5);
	println!("this takes an i32: {x}");
}

fn print_something(x: i32)  {
	println!("this is something {x}");
}

fn return_value(x: f64) -> f64 {
	x
}

fn two_parameters(unit_label: char, hex: i32) {
	println!("the two parameters are {unit_label}{hex}");
}

fn return_early(x: char) -> char {
	return x;
}

fn i_32(x: i32) -> i32 {
	let x = {
		let y = x;
		y-2
	};
	x
}
