use std::io;

fn trapezium_area(height: f64, base1: f64, base2: f64) {
    let area = (height / 2.0) * (base1 + base2);
    println!("The area of a trapezium is : {}", area);
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) {
    let area = 0.5 * (diagonal1 * diagonal2);
    println!("The area of a rhombus is : {}", area);
}

fn parallelogram_area(base: f64, altitude: f64) {
    let area = base * altitude;
    println!("The area of a parallelogram is : {}", area);
}

fn cube_surface_area(side: f64) {
    let surface_area = 6.0 * side * side;
    println!("The surface area of a cube is : {}", surface_area);
}

fn cylinder_volume(radius: f64, height: f64) {
    let pi = 22.0 / 7.0;
    let volume = pi * radius * radius * height;
    println!("The volume of a cylinder is : {}", volume);
}

fn main() {
    println!("What shape are you trying to calculate (trapezium(t), rhombus(r), parallelogram(p), cube(cu), cylinder(cy)):");
    
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read");
    let option = input1.trim().to_lowercase();

    if option == "t" || option == "trapezium" {
        println!("Enter height:");
        let mut input_height = String::new();
        io::stdin().read_line(&mut input_height).expect("Failed to read");
        let height: f64 = input_height.trim().parse().expect("Invalid input");

        println!("Enter base1:");
        let mut input_b1 = String::new();
        io::stdin().read_line(&mut input_b1).expect("Failed to read");
        let base1: f64 = input_b1.trim().parse().expect("Invalid input");

        println!("Enter base2:");
        let mut input_b2 = String::new();
        io::stdin().read_line(&mut input_b2).expect("Failed to read");
        let base2: f64 = input_b2.trim().parse().expect("Invalid input");

        trapezium_area(height, base1, base2);

    } else if option == "r" || option == "rhombus" {
        println!("Enter diagonal 1:");
        let mut input_d1 = String::new();
        io::stdin().read_line(&mut input_d1).expect("Failed to read");
        let diagonal1: f64 = input_d1.trim().parse().expect("Invalid input");

        println!("Enter diagonal 2:");
        let mut input_d2 = String::new();
        io::stdin().read_line(&mut input_d2).expect("Failed to read");
        let diagonal2: f64 = input_d2.trim().parse().expect("Invalid input");

        rhombus_area(diagonal1, diagonal2);

    } else if option == "p" || option == "parallelogram" {
        println!("Enter base:");
        let mut input_base = String::new();
        io::stdin().read_line(&mut input_base).expect("Failed to read");
        let base: f64 = input_base.trim().parse().expect("Invalid input");

        println!("Enter altitude:");
        let mut input_alt = String::new();
        io::stdin().read_line(&mut input_alt).expect("Failed to read");
        let altitude: f64 = input_alt.trim().parse().expect("Invalid input");

        parallelogram_area(base, altitude);

    } else if option == "cu" || option == "cube" {
        println!("Enter side:");
        let mut input_side = String::new();
        io::stdin().read_line(&mut input_side).expect("Failed to read");
        let side: f64 = input_side.trim().parse().expect("Invalid input");

        cube_surface_area(side);

    } else if option == "cy" || option == "cylinder" {
        println!("Enter radius:");
        let mut input_rad = String::new();
        io::stdin().read_line(&mut input_rad).expect("Failed to read");
        let radius: f64 = input_rad.trim().parse().expect("Invalid input");

        println!("Enter height:");
        let mut input_h = String::new();
        io::stdin().read_line(&mut input_h).expect("Failed to read");
        let height: f64 = input_h.trim().parse().expect("Invalid input");

        cylinder_volume(radius, height);

    } else {
        println!("Invalid shape option entered.");
    }
}