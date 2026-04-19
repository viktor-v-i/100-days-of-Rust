enum Coin {
	Penny,
	Dime,
	Quarter(String)
}

fn main() {
	let coin = Coin::Quarter(String::from("Alaska"));	
	let penny = Coin::Penny;
	
	let config_max: Option<u8> = Some(3);
	if let Some(max) = config_max {
		println!("The maximum is configured to {max}");
	}
	let mut count: u32 = 0;
	if let Coin::Quarter(state) = coin {
		println!("State quarter from {state}!");
	} else {
		count += 1;
	}
	println!("{count}");
		
	let mut count: u32 = 0;
	if let Coin::Quarter(state) = penny {
		println!("State quarter from {state}!");
	} else {
		count += 1;
	}
	println!("{count}");

    let quarter = describe_quarter(Coin::Quarter(String::from("Alaska")));
    let dime = describe_quarter(Coin::Dime);

	println!("{quarter:?}");
	println!("{dime:?}");
}

fn describe_quarter(coin: Coin) -> Option<String> {
	let Coin::Quarter(state) = coin else { return None; };
		Some(format!("Quarter from {state}"))
}
