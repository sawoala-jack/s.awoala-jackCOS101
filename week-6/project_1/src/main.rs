use std::io;

fn main() {
    println!("Are you ready to order (yes or no)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice = input1.trim().to_lowercase();

    if choice == "yes" {
        println!(
"The Food available and their prices:
  Poundo Yam/EdinKaiko Soup(p) : 3200
  Fried Rice & Chicken(f)      : 3000
  Amala & Ewedu Soup(a)        : 2500
  Eba & Egusi Soup(e)          : 2000
  White Rice & Stew(w)         : 2500"
        );
    } else if choice == "no" {
        println!("I understand, take ur time");
        return;
    } else {
        println!("Invalid choice.");
        return;
    }

    let mut cumulative_total = 0.0;

    loop {
        println!("What would you like to order? Enter the first letter (p, f, a, e or w):");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let order = input2.trim().to_lowercase();

        let (_food_name, unit_price): (&str, f64) = match order.as_str() {
            "p" => ("Poundo Yam/EdinKaiko Soup", 3200.00),
            "f" => ("Fried Rice & Chicken", 3000.00),
            "a" => ("Amala & Ewedu Soup", 2500.00),
            "e" => ("Eba & Egusi Soup", 2000.00),
            "w" => ("White Rice & Stew", 2500.00),
            _ => {
                println!("Invalid food code!");
                return;
            }
        };

        println!("How many portions do you want?");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let portions: u32 = input3.trim().parse().expect("Integer is a failed value");

        let total_cost = unit_price * (portions as f64);
        cumulative_total += total_cost;

        println!("Do you want to order anything else? (yes or no)");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let order_more = input4.trim().to_lowercase();

        if order_more == "yes" {
            // Continues the loop to order another item
            continue;
        } else if order_more == "no" {
            break;
        } else {
            println!("Invalid choice. Please enter either yes or no.");
            return;
        }
    }

    let total_cost = cumulative_total;

    if total_cost > 10_000.00 {
        let final_cost = (total_cost) - (0.05 * total_cost);
        println!("You have a discount of 5% available.");
        println!("Your final total is N{}", final_cost);
    } else {
        let final_cost = total_cost;
        println!("Your final total is N{}", final_cost);
    }
}