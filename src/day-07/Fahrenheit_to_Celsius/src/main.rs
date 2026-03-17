const FREEZING_FAHRENHEIT: f64 = 32.0;

fn main() {
	let fahrenheit = [-10.0, 32.0, 72.0, 90.0, 100.0]; 
	
		for temp_f in fahrenheit {
			let fahrenheit = f_to_c(temp_f);
			if fahrenheit < 0.0 {
				println!("freezing");
			} else if fahrenheit >= 20.0 && fahrenheit <= 25.0 {
				println!("comfortable");
			} else if fahrenheit > 35.0 {
				println!("hot");
			} else {
				println!("outside scope");
			}
			println!("the converstion is of f {temp_f} to c {fahrenheit}");
	}


	let celsius = [-15.0, 0.0, 22.0, 30.0, 40.0];
		
		for temp_c in celsius {
			let celsius = c_to_f(temp_c);
				if celsius < FREEZING_FAHRENHEIT {
					println!("freezing");
				} else if celsius >= 68.0 && celsius <= 77.0 {
					println!("comfortable");
				} else if celsius > 95.0 {
					println!("hot");
				} else {
					println!("outside scope");
				}
			println!("the conversion is of c {temp_c} to f {celsius}");	

		}

}

fn f_to_c(f: f64) -> f64 {
	let fahrenheit = (f - FREEZING_FAHRENHEIT) * 5.0 / 9.0;	
		fahrenheit
} 
	


fn c_to_f(c: f64) -> f64 {
	let celsius = c * 9.0 / 5.0 + FREEZING_FAHRENHEIT;
		celsius
}
