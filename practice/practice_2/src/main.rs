use std::io;

fn facbub(){
    let mut name = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    let mut papers = String::new();
    println!("How many papers have you published?");
    io::stdin().read_line(&mut papers).expect("Not a valid string");
    let papers:i32 = papers.trim().parse().expect("Not a valid answer");

    println!(" Name:{}\nNumber of papers published:{}",name,papers );

    if papers >=3 && papers < 5{
        println!("Your incentitive is N500,000");
    }
    else if papers >5 && papers < 10{
        println!("Your incentitive is N800,000");
    }
    else if papers >= 10{
        println!("Your incentitive is N100,000,000");
    }
    else {
        println!("Your incentitive is N300,000");
    }
    
}
fn main() {
    println!("Faculty publication System");
    facbub()
}
