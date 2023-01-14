use std::io::Write;

fn main() {
    let lager =
    "33 Export
    Desperados
    Goldberg
    Gulder
    Heineken
    Star\n";
    let stout = 
    "Legend
    Turbo King
    Williams\n";
    let non_alcoholic =
    "Maltina
    Amstel Malta
    Malta Gold
    Fayrouz";

    let mut file = std::fs::File::create("Nigeria Breweries.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Brewery Limited\n"
        .as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("\nData written to file.");
}
