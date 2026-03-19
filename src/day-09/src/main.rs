fn main() {

	// makes a copy
	// fixed scalars implement the copy trait, gets pushed to the stack 
	let r: i32 = 5;
	let t: i32 = r;
	println!("{r} & {t}"); 
	
	// same as above
	let c: f64 = 32.0;
	let v: f64 = c;
	println!("{c} & {v}");

	// same as above
	let x: bool = true;
	let y: bool = x;
	println!("{x} & {y}");


	//	let s1 = String::from("move");
	//	let s2 = s1;
	//	println!("{s1} now!");
	// move happens because the String doesn't implement the copy trait
	

	// this is a move, string doesn't implement the copy trait
	// gets pushed to the heap because its size can grow and shrink at run time
	let s1 = String::from("move");
	let s2 = s1;
	println!("{s2}, world!");

	// copies the heap data of the string
	let s1 = String::from("clone");
	let s2 = s1.clone();
	println!("{s2} is a clone of {s1}");
}
