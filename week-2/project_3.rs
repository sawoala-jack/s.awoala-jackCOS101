fn main(){
	let n:f64 = 3.0;
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;

	// Depriciated value
	let a = p * (1.0 - (r/100.0)).powf(n);
	println!("Depriciated value after 3 years is {:.2}", a);
}