fn main() {
	let x = Some(5);
	let y = Some('a');
	let z: Option<i32> = None;
	println!("{x:?} \n{y:?} \n{z:?}");
	
    // let add: i32 = 5;
	
	// Option<i32> is a wrapper type not an i32. + is defined for same types-
	// to add them you'd first need to extract the inner value and handle the None case
	// let sum = add + x;  
}
