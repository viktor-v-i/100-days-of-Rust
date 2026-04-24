// Module hosting was decleard without pub which makes it private. Even if      
// front_of_house can see its own child hosting, code outside front_of_house     
// cannot.
fn eat_at_restaurant() {
	front_of_house::hosting::add_to_waitlist();
}

mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}
	
		fn seat_at_table() {}
	
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}
	
		fn take_payment() {}
	
	}
}
