// Calculate TV price after 3 years 
fn main() {
	let p:f64 = 510_000.0;  // principal
	let r:f64 = 5.0;        // rate in percentage
	let t:f64 = 3.0;        // time in years

	// Amount after 4 years
	let a = p * (1.0 - (r/100.0)).powf(t);
	println!("The price of the TV after 3 years is ₦{:.2}", a)
}