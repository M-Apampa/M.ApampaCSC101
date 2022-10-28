fn main(){
	let p:f64= 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// compound interest
	let b = 1.0 + (r / 100.00);
	let c = b.powf(n);
	let a = p * c;
	let ci = a - p;
	println!("Compound interest is {}",ci );

}