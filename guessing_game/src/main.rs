use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("{}", "Please input your guess number");

	let secret_number = rand::thread_rng().gen_range(0..101);

	loop {
		let mut guess = String::new();
		
		io::stdin().read_line(&mut guess).expect("Can't read from stdin");

		let guess: i32 = guess.trim().parse().expect("Please type a number");
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("{}", "Less"),
			Ordering::Greater => println!("{}", "Greater"),
			Ordering::Equal => {
				println!("{}", "You win");
				break;
			}
		}
	}
}

