enum TrafficLight {
	Red,
	Yellow,
	Green
}

enum IpAddrKind {
	V4(String),
	V6(String)
}

fn main() {
	let red = TrafficLight::Red;
	let yellow = TrafficLight::Yellow;
	let green =	TrafficLight::Green;
	
	describe(red);
	describe(yellow);
	describe(green);
	
	let home = IpAddrKind::V4( String::from("127.0.0.1"));
	
	let loopback = IpAddrKind::V6( String::from("::1"));

}

fn describe(light: TrafficLight) {
	println!("describe called");
}

