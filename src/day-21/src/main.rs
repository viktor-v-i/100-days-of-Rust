#[derive(Debug)]
struct Item {
	name: String,
	quantity: u32,
	price_cents: u32
}

impl Item {
	fn new(name: String, quantity: u32, price_cents: u32) -> Self {
		Self {
			name,
			quantity,
			price_cents
			 }
	}

	fn total_price(&self) -> u32 {
		self.quantity * self.price_cents
	}

	fn restock(&mut self, amount: u32) {
		self.quantity += amount
	}

	fn is_expensive(&self) -> bool {
		self.price_cents > 1000
	}
}

struct CategoryId(u32);


fn main() {
	let mut item = Item::new(String::from("Sensor"), 32, 100);
	println!("\nNew item: {item:#?}");

	println!("\nTotal Price: {}", item.total_price());
	println!("Is expensive? {}", item.is_expensive());

	item.restock(50);
	println!("Total price after restock: {}", item.total_price());   

	let second_item = Item {
		name: String::from("Actuator"),
		..item
	};
	println!("\nOriginal item: {}", item.name);
	println!("Second item: {}", second_item.name);

	let id = CategoryId(15);
	println!("\nCategory ID: {}", id.0);
}
