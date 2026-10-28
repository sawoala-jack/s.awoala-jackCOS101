struct Laptop {
    brand:String,
    quantity:u32,
    price:u32
}
fn main() {
    let lap1 = Laptop{
        brand:String::from("HP"),
        quantity:10,
        price:650_000
    };
    let lap2 = Laptop{
        brand:String::from("IBM"),
        quantity:6,
        price:755_000
    };
    let lap3 = Laptop{
        brand:String::from("Toshiba"),
        quantity:10,
        price:550_000
    };
    let lap4 = Laptop{
        brand:String::from("Dell"),
        quantity:4,
        price:850_000
    };
    println!("These are the laptops we have available");
    display(&lap1);
    display(&lap2);
    display(&lap3);
    display(&lap4);
    let mut total_bought: u32 = 0;
    let mut grand_total: u32 = 0;

    loop {

        println!("Please enter the laptop you want to buy");
        let mut input1 = String::new();
        std::io::stdin().read_line(&mut input1).expect("failed to put a valid number");
        let brand_name:String = input1.trim().parse().expect("Please enter a valid name");

        if brand_name == "HP" || brand_name == "IBM" || brand_name == "Toshiba" || brand_name == "Dell"{
            println!("Ok noted");
            println!("Please enter the amount of laptops you want to buy");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).expect("failed to put a valid number");
            let number:u32 = input2.trim().parse().expect("failed to read input");

            total_bought = total_bought + 1;
            let price = if brand_name == "HP" {
                lap1.price
            } else if brand_name == "IBM" {
                lap2.price
            } else if brand_name == "Toshiba" {
                lap3.price
            } else {
                lap4.price
            };

            grand_total = grand_total + (number * price);
        }
        else{
            println!("that is not sold here");
        }
        println!("Any other laptops in mind (keep in mind u need to buy atleast 3 laptops to cah out) (Yes or No)");
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("invalid input");
        let choice:String = input3.trim().to_lowercase();
        
        if choice == "yes" || choice == "y" {
            continue;
        }
        else if (choice == "no" || choice == "n")&& total_bought >= 3 {
            println!("Ok we are going to process ur bill for you");
            println!("Your grand total comes out to : {} naira", grand_total );
            break;
        }
        else if choice == "no" || choice == "n" {
            println!("ok, but u need three laptops to leave. You currently have {}", total_bought);
        } 
        else {
            println!("ok, but u need three laptops to leave");
        }
    }
}
fn display(lap: &Laptop){
    println!("Laptop name :{}, Quantity left: {}, Price per unit: ₦{}"
        ,lap.brand, lap.quantity, lap.price);
}