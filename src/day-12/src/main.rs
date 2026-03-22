fn main() {

	let mut s = String::from("mutable");
	println!("{s}");

	string(&mut s); 
	println!("{s}");

	// error will occur because we cannot borrow a mutable reference mmore than once
	
	//let a1 = &mut s;
	//let a2 = &mut s;
	//println!("{a1}, {a2}");

	// if you want to mutate multiple references we can do so by applying a scope
	{
		let a1 = &mut s;
		println!("{a1}");
	}	// a1 goes out of scope so we can create  a new reference


	let a2 = &mut s;
	println!("{a2}");
	
	//let b1 = &s;
	//let b2 = &mut s;
	//println!("{b1}, {b2}");
	// cannot borrow s as mutable because it's already borrowed as immutable


	let b1 = &s;
	println!("{b1}");
	//scope of variable ends here

	let b2 = &mut s;
	println!("{b2}");
}

fn string(variable: &mut String) {

	variable.push_str(", this is");
}
