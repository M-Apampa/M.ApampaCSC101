fn main(){
	let s:f64 = 450_000.00 + 1_500_000.00 + 750_000.00 + 2_850_000.00 + 250_000.00;
	let n:f64 = 10.0;

	//sum and average
	println!("The sum of the sales record is {}",s);
	let average = s / n;
	println!("The average of the sales record is {}",average);



}