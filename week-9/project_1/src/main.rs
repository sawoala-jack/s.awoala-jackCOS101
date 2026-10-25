use std::io::Write;

fn main() {
    let mut csv = std::fs::File::create("Breweries.csv").expect("create failed");

    csv.write_all(b"LAGER, STOUT, NON-ALCOHOLIC\n").expect("Failed to write");
    csv.write_all(b"33 Export, Legend, Maltina\n").expect("Failed to write");
    csv.write_all(b"Desperados, Turbo King, Amastel Malta\n").expect("Failed to write");
    csv.write_all(b"Goldberg, Williams, Malta Gold\n").expect("Failed to write");
    csv.write_all(b"Gulder, , Fayrouz\n").expect("Failed to write");
    csv.write_all(b"Heineken, , \n").expect("Failed to write");
    csv.write_all(b"star, , \n").expect("Failed to write");
  
    println!("Data has been written, it is now a csv file!");
}