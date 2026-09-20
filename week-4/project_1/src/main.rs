use std::io;

fn main() 
{
    let mut run = true;

  while run {
        println!("Enter the value of x²");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Please put in an appropriate value next time");
        let a:f64 = input1.trim().parse().expect("Please put in an appropriate value next time");

        println!("Enter the value of x");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Please put in an appropriate value next time");
        let b:f64 = input2.trim().parse().expect("Please put in an appropriate value next time");

        println!("Enter the value of  the constant");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Please put in an appropriate value next time");
        let c:f64 = input3.trim().parse().expect("Please put in an appropriate value next time");

        // find d
        let d:f64 = b*b - 4.0 * a * c ;

        if d > 0.0 {
            let _x1:f64 = (-b + d.sqrt()) / 2.0*(a);
            let _x2:f64 = (-b - d.sqrt()) / 2.0*(a);
            println!("The roots of x are: x1 = {}, x2 = {}",_x1,_x2 );
        }    
        else if d == 0.0 {
            let _x1:f64 = -b / 2.0*(a);
            println!("The root of x is: {}",_x1 );
        }        
        else if d < 0.0 {
            println!("The value x has no real root because d < 0");
        }            
        println!("Do you want to run another quadratic calculation (yes or no)");
        let mut chose = String::new();
        io::stdin().read_line(&mut chose).expect("Please input an accurate option");
        let options = chose.trim().expect;
        run = options == "yes" || options == "y";
    }
    println!("OK THANK YOU AND GOODBYE");
     
}   