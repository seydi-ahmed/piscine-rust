// use borrow_me_the_reference::{delete_and_backspace, do_operations};
use borrow_me_the_reference::*;

fn main() {
	let mut a = String::from("bpp--o+er+++sskroi-++lcw");
	// let mut b: Vec<String> = vec!["2+2", "3+2", "10-3", "5+5"];

	// delete_and_backspace(&mut a);
	// do_operations(&mut b);

    println!("{:?}", delete_and_backspace(&mut a));
	// println!("{:?}", (a, b));
}