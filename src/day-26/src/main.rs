fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
	println!("{six:?} \n{none:?}");

	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => { 
			println!("moved to space {other:?}");
			move_player(other)
		}
	}  
	match dice_roll {
		3=> add_fancy_hat(),
		7=> remove_fancy_hat(),
		_ => {
			println!("reroll");
			reroll()	
		}

	}
	
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

// error[E0004] : non-exhuastie pattern: None not covered
//fn plus_one_buggy(x: Option<i32>) -> Option<i32> {
//	match x {
//		Some(i) => Some(i + 1),
//	}
//}
