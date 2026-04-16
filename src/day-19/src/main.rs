#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	
	fn is_square(&self) -> bool {
		self.width == self.height
	}
	
	fn widen(&mut self, width: u32)  {
		self.width = self.width + width
	}
}



fn main() {
	let mut rect = Rectangle {
		width: 30,
		height: 50,
		};

	println!("Area: {}", rect.area());
	println!("Is Square: {}", rect.is_square());
	
	rect.widen(10);
	println!("Area after widening by 10: {}", rect.area());

}
