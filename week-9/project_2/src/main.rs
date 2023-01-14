use std::io::Write;

fn main() {
    let pau_sims =
    "Student Name        Matric.Number  Department   Level
     Oluchi Mordi        ACC10211111   Accounting   300
     Adams Aliyu         ECO10110101   Economics    100
     Shania Bolade       CSC10328828   Computer     200
     Adekunle Gold       EEE11020202   Electrical   200
     Blanca Edemoh       MEE10202001   Mechanical   100\n";
    

    let mut file = std::fs::File::create("PAU-SIMS.txt").expect("create failed");
    file.write_all("Welcome to PAU-SIMS\n"
        .as_bytes()).expect("write failed");
    file.write_all(pau_sims.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}
