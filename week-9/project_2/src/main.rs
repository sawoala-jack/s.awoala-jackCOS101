use std::io::Write;

fn main() {
    let pau_smis: Vec<(&str, &str, &str, &str)> = vec![
        ("Student Name", "Matric Number", "Department", "Level"),
        ("Oluchi Mordi", "ACC10211111", "Accounting", "100"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Bianca Edomoh", "MEE10202001", "Mechanical", "100"),
    ];

    let mut csv = std::fs::File::create("PAU-SMIS.csv").expect("the csv file u create failed");
    for student in pau_smis{
        println!("Name: {}, Matric No: {}, Dept: {}, Level: {}",student.0, student.1, student.2, student.3);
        let line = student.0.to_string() + "," + student.1 + "," + student.2 + "," + student.3 + "\n";
        csv.write_all(line.as_bytes()).expect("YOUR file failed");
    }
    println!("your file has been made")

}