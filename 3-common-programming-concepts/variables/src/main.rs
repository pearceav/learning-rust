fn main() {

	// Mutability
	// let mut x = 5;
	// println!("The value of x is: {x}");
	// x = 6;
	// println!("The value of x is: {x}");
	
	// Shadowing
	let x = 5;
	
	let x = x + 1;
	
	{
		let x = x*2;
		println!("The value of x in the inner scope is: {x}");
	}
	
	println!("The value of x is: {x}");
	
	// This works because of shadowing 
	let spaces = "    ";
	let spaces = spaces.len();
	
	println!("The value of spaces is: {spaces}");
	
	// This does not work bc its changing the variable type
	// let mut spaces = "    ";
	// spaces = spaces.len();
	
}
