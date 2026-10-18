fn main() {
    let mut interview:Vec<(String, String, u32)> = vec![];

    println!("How many candidates are you inputting");
    let mut input5 = String::new();
    std::io::stdin().read_line(&mut input5).expect("Failed to read your input");
    let amount:u32 = input5.trim().parse().expect("This input was failed to read");

    for i in 0..amount{
        println!("Please Kindly Enter the candidate Full Name (Last Name  First Name)");
        let mut input1 = String::new();
        let mut input2 = String::new();
        println!("Please enter candidate {} the first name", i+1);
        std::io::stdin().read_line(&mut input1).expect("Failed to read your input");
        println!("Please enter candidate {} the last name", i+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read your input");
        let first_name = input1.trim().to_string();
        let last_name = input2.trim().to_string();

        println!("Please enter the years of experience for candidate {} {}:", last_name, first_name);
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("Failed to read your input");
        let years:u32 = input3.trim().parse().expect("This input was failed to read");

        interview.push((last_name, first_name, years))
    }

    let interview_slice = &interview[..];

    let mut highest_years:u32 = 0;
    let mut top_last_name = "";
    let mut top_first_name = "";

    for candidate in interview_slice{
        if candidate.2 > highest_years{
            highest_years = candidate.2;
            top_last_name = &candidate.1;
            top_first_name = &candidate.0;
        }
    }
    if highest_years > 0 {
        println!("The candidate with the highest experience is {} {} with a total amount of {} years",top_last_name,top_first_name, highest_years );
    }
    else {
        println!("You didnt enter any candidate records");
    }
}
