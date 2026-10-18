
fn main() {
    let job_table: Vec<(&str, u32, u32, &str, &str, &str, &str)> = vec![
        ("APS 1-2", 1, 2, "Intern", "-", "Paralegal", "Placement"),
        ("APS 3-5", 3, 5, "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", 5, 8, "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"),
        ("EL1 8-10", 8, 10, "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", 10, 13, "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", 14, 99, "CEO", "Dean", "Partner", "Principal"),
    ];

    println!("Enter your job title:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input accurately");
    let search = input1.trim().to_lowercase();

    let records_sl = &job_table[..];
    let mut result = "";

    for record in records_sl {
        if record.3.to_lowercase() == search || record.4.to_lowercase() == search || record.5.to_lowercase() == search || record.6.to_lowercase() == search
        {
            result = record.0;
        }
    }

    if result != "" {
        println!("APS Level: {}", result);
    } else {
        println!("Your Job title is not around.  Please Enter years of experience:");
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let years: u32 = input2.trim().parse().expect("Next time enter a real input please");

        result = "";

        for record in records_sl 
        {
            if years >= record.1 && years <= record.2 
            {
                result = record.0;
            }
        }

        if result != "" 
        {
            println!("APS Level: {}", result);
        } 
        else 
        {
            println!("No APS level found for {} years.", years);
        }
    }
}