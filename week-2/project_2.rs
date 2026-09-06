fn main(){
	let ato:f64 = 450_000.00;
	let ama:f64 = 1_500_000.00;
	let ahp:f64 = 750_000.00;
	let ade:f64 = 2_500_000.00;
	let aac:f64 = 250_000.00;
	let qto:f64 = 2.0;
	let qma:f64 = 1.0;
	let qhp:f64 = 3.0;
	let qde:f64 = 3.0;
	let qac:f64 = 1.0;

	let qty = qto + qma + qhp + qac + qde; 

	// Sum of the sales
	let sum = (ato * qto) + (ama * qma) + (ahp * qhp) + (ade * qde) + (aac * qac); 
	println!("The sum of the sales is {}", sum);

	// Average sales
	let ave = sum / qty;
	println!("The average is {}", ave);	
}