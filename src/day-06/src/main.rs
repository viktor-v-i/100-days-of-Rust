fn main() {
	let mut number = 1;
		let count = loop {
			println!("remaining = {number}");
			if number % 3 == 0 && number % 7 == 0 {
				break number;
			} else {
			number +=1
			}
		};
		println!("End count is {count}");

	lift_off();
	array();
	array_2();
	bonus();
}	

fn lift_off() {
	let mut count_down = 10;
	
	while count_down !=0 {
		println!("{count_down}!");

		count_down -=1
	}
	
	println!("LIFTOFF!");

}

fn array() {
	let array = [5, 4, 3, 2, 1];

	for index in array {
		println!("the value is: {index}");
	 }
}

fn array_2() {
	for count in 1..6 {
		println!("{count}");
	}
		println!("end");
}

fn bonus() {
	// example from book
	let mut nest = 0;
	'nested: loop {
		println!("count = {nest}");
		let mut remaining = 25;

		loop {
			println!("remaining = {remaining}");
			if remaining == 20 {
				break;
			}
			if nest == 5 {
				break 'nested;
			}
			remaining -= 5;

		}

		nest += 5

	}
	println!("end count = {nest}");
}	
