#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

fn main() {
	let rect = Rectangle {
	width: dbg!(30),
	height: 50
	};   	
	println!("\nDebug: rect is {rect:?}");
	
	println!("\ndbg!");	
	dbg!(&rect);

	println!("\nPretty: {rect:#?}");
	
	let area = area(&rect);
	println!("\nArea: {area}");
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height	
}
