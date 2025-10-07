// Chief Donatus and Sons Ltd sum and average of sales 
fn main() {
	// Sales amounts 
	let toshiba:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	// Quantities sold 
	let toshiba_q:f64 = 2.0;
	let mac_q:f64 = 1.0;
	let hp_q:f64 = 3.0;
	let dell_q:f64 = 3.0;
	let acer_q:f64 = 1.0;

	// Total sum of sales
	let total_sales = toshiba + mac + hp + dell + acer;
	println!("The Sum of the Sales Record is ₦{:.2}", total_sales);

	// Total quantity
	let total_qty = toshiba_q + mac_q + hp_q + dell_q + acer_q;

	// Average sales
	let average = total_sales / total_qty;
	println!("The Average of the Sales Record is ₦{:.2}", average);
}