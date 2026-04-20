#[derive(Debug, Clone)]
enum Shape {
	Circle(f64),
	Rectangle { width: f64, height: f64 },
	Triangle(f64, f64, f64)
}

impl Shape {
	fn area(&self) -> f64 {
	match self {
		Shape::Circle(r) => 3.14159 * r * r,
		Shape::Rectangle { width, height } => width * height,
		Shape::Triangle(_, _, _) => 0.0
		}
	}
	fn describe(&self) -> String {
	match self {
		Shape::Circle(r) => format!("Circle with radius {r}"),
		Shape::Rectangle { width, height } => format!("Rectangle {width}x{height}"),
		Shape::Triangle(a, b, c) => format!("Triangle {a}-{b}-{c}")
		}
	}

}

fn print_header(s: &Shape) {
	println!("--- shape ---");
}

fn main() {
	let shapes = [Shape::Circle(5.0), Shape::Rectangle { width: 4.0, height: 6.0 }, Shape::Triangle(3.0, 4.0, 5.0)];
	for shape in &shapes {
		print_header(shape); // ()
		println!("{} {}", shape.describe(), shape.area()); // describe() -> String, area() -> f64, println! -> ()
		}

	if let Some(winner) = biggest_area(&shapes) { // biggest_area -> Option<Shape> 
		println!("Biggest: {:?}", winner); // ()
	} else {
		println!("no shapes"); // ()
	}
	
	let empty: [Shape; 0] = [];
	println!("{:?}", biggest_area(&empty)); // biggest_area -> Option<Shape>, println! -> ()
}

fn biggest_area(shapes: &[Shape]) -> Option<Shape> {
	if shapes.is_empty() { // Option<Shape> 
		 return None; 
	} 

	let mut biggest = &shapes[0];
	for shape in &shapes[1..] {
		if shape.area() > biggest.area() {
			biggest = shape;
			}
		}
	Some(biggest.clone()) 
}
