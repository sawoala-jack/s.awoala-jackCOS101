fn main(){
	let n:f64 = 5.0;
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;

	// Compound Interest
	let a = p * (1.0 + (r/100.0)).powf(n);
	println!("Compound amount {}", a);

	let ci = a - p;
	println!("Compound Interest is {}", ci);
}