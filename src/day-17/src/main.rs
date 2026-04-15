struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Placeholder;

fn main() {

	let white = Color(255, 255, 255);
    let origin = Point(90, 90, 90);
	println!("Color: {} {} {}", white.0, white.1, white.2);
    println!("Point: {} {} {}", origin.0, origin.1, origin.2);

	let Point(x, y, z) = origin;
	println!("Deconstructed: x={} y={} z={}", x, y, z);

	let _attempt = Placeholder;
	//exists as a type

	describe_color(&white) //making a reference to Color
}
						//type, this paramenter accepts a reference to Color
fn describe_color(color: &Color) {
	println!("Color description: R={} G={} B={}", color.0, color.1, color.2);


}
