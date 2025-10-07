fn main() {
	let p:f64 = 520_000_000.0;  // principal
	let t:f64 = 5.0;          // time in years 
	let r:f64 = 10.0;         // rate in percentage 

	// Formula: A = P *[1 + R/100]^t
	let a = p*(1.0 + (r/100.0)).powf(t);
	println!("The amount after 5 years is ₦{:.2}", a);

	// Compound Interest 
	let ci = a - p;
	println!("The Compound Interest is ₦{:.2}", ci);
}