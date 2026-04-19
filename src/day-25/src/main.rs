enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(String)
}

fn main() {
	
	let penny = value_in_cents(Coin::Penny);
	let nickel = value_in_cents(Coin::Nickel);
	let dime = value_in_cents(Coin::Dime);	

	println!("Penny: {penny} \nNickel: {nickel} \nDime: {dime}");
	
	let quarter = value_in_cents(Coin::Quarter(String::from("Alaska")));
	println!("Quarter: {quarter}");
	// value_in_cents(Coin::Quarter(String::from("Alaska")));

}	

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State Quarter from {state}!");
			25
		}
	}	
}
