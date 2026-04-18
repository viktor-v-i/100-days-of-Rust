enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32)
}

impl Message {
	fn call(&self) {
	println!("call invoked");
	}
}

fn main() {
	let quit = Message::Quit;
	quit.call();
	let m_move = Message::Move{ x: 20, y: 10 };
	m_move.call();	
	let write = Message::Write(String::from("Write"));
	write.call();
	let color = Message::ChangeColor(40, 50, 20);
	color.call();
	
}
