use std::io::Write;

fn main() {
    let commissioner = vec![
        ("Aigbogun Alamba Daudu"),
        ("Murtala Afeez Bendu"),
        ("Okorocha Calistus Ogbona"),
        ("Adewale Jimoh Akanbi"),
        ("Osazuwa Faith Etieye"),
    ];
    let ministry = vec![
        ("Internal Affairs"),
        ("Justice"),
        ("Defense"),
        ("Power & Steel"),
        ("Petroleum"),
    ];
    let zone = vec![
        ("South West"),
        ("North East"),
        ("South South"),
        ("South West"),
        ("South East"),
    ];
    let mut file = std::fs::File::create("EFCC_Merge.csv").expect("the csv file u create failed");
    for i in 0..commissioner.len(){
        let comm = commissioner[i];
        let min = ministry[i];
        let zo = zone[i];

        let line = comm.to_string() + "," + min + "," + zo  + "\n";
        file.write_all(line.as_bytes()).expect("IT Failed bruh");
    }
    println!("\nThe tables have been merged");
}
