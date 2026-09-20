use std::io;

fn main() {
    println!("Please input the amount of years you have worked");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please put in an appropriate value next time");
    let years_worked: f32 = input1.trim().parse().expect("Please put in an appropriate value next time");

    println!("Please input your age");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Please put in an appropriate value next time");
    let b: i32 = input2.trim().parse().expect("Please put in an appropriate value next time"); 

    let i1: f64 = 1_560_000.00; 
    let i2: f64 = 1_480_000.00;
    let i3: f64 = 1_300_000.00; 
    let i4: f64 = 100_000.00;

    
    let status = if years_worked >= 4.0 {
        println!("You are Experienced");
        "Experienced"
    } else {
        println!("You are Not Experienced");
        "Not Experienced"
    };

    
    if status == "Experienced" && b >= 40 {
        println!("Your incentive is: N{}", i1);
    } 
    else if status == "Experienced" && b >= 30 && b == 39 {
        println!("Your incentive is: N{}", i2);
    } 
    else if status == "Experienced" && b <= 29 {
        println!("Your incentive is: N{}", i3);
    } 
    else if status == "Not Experienced" {
        println!("Your incentive is: N{}", i4);
    }
}