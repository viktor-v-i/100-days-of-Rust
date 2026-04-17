#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
	}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size
			}
	 }
}





fn main() {
	let rect = Rectangle {
		width: 30,
		height: 50
		};
	
	let rect1 = Rectangle {
		width: 40,
		height: 60
		};
	
	println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
	println!("Can rect1 hold rect? {}", rect1.can_hold(&rect));
	
	let sq = Rectangle::square(3);
	println!("Square: {sq:?}");
}
