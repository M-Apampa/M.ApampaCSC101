fn main(){
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//calculate depreciation
	let b = 1.00 - (r / 100.00);
	let c = b.powf(n);
	let a = p * c;
	println!("The depreciation value after 3 years is {}",a );

}