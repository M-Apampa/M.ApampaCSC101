// Rust program to determine annual incentive

use std::io;

fn main() {
     
    //input whether experienced or not
    println!("\nHow many years of experience do you have?");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Not a valid number");
    let experience:i32 = experience.trim().parse().expect("Not a valid number");

    println!("How old are you?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Not a valid number");
    let age:i32 = age.trim().parse().expect("Not a valid number");

    if experience>=15 && age>=40{
        println!("Your annual salary is N1,560,000.");
    }else  if experience>=15 && age>=30{
        println!("Your annual salary is N1_480_000");
    }else if experience>=15 && age <28{
        println!("Your annual salary is N1,300,000.");
    }else{
        println!("Your annual salary is N100,000.");
    }
}
