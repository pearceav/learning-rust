use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

	// Code that gets a guess from the user and prints it
	println!("Guess the number!");
	
	let secret_number = rand::thread_rng().gen_range(1..=100);
	
	//println!("The secret number is: {secret_number}");
	
	loop {
		println!("Please input your guess.");
		
		let mut guess = String::new();
		
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		
		// Shaddows the guess variable to a number type
		//let guess: u32 = guess.trim().parse().expect("Please type a number!");
		
		// Handling invalid input
		let guess:u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
			
		println!("You guessed: {guess}");
		
		// Handling the possible return values of comparing two numbers
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
