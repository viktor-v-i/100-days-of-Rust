fn main() {
   let number = 21;
	
	if number > 0 {
		println!("number is positive");
		} else if number < 0 {
		println!("number is negative");
		} else if  number == 0 {
		println!("number is zero");
		}

	if number % 2 == 0 {
		println!("number is even");					
		} else {
		println!("number is odd");
		}

	if number % 3 == 0 && number % 5 == 0 {
		println!("number is divisible by 3 and 5");
		} else if number % 3 == 0 && number % 5 != 0 {
		println!("number is divisible by 3 not 5");
		} else if number % 3 != 0 && number % 5 == 0 {
		println!("number is divisible by 5 not 3");
		} else {
		println!("number is divisible by neither");
		}

	condition();
}

fn condition() {
	let condition = false;
	let number = if condition { 2 } else {10};

	println!("the value of the number is: {number}"); 

}
