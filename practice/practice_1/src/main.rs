use std::io;

fn main() {
    for x in 0..15{
let mut name = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    let mut email = String::new();
    println!("Input your email address");
    io::stdin().read_line(&mut email).expect("Not a valid string");

     let mut department = String::new();
    println!("Which department are you?");
    io::stdin().read_line(&mut department).expect("Not a valid string");

     let mut state = String::new();
    println!("Which state are you from?");
    io::stdin().read_line(&mut state).expect("Not a valid string");

    let mut rep = String::new();
    let mut level = String::new();
    let mut gpa = String::new();
    println!("If you are a class rep press 1, if not press 0");
    io::stdin().read_line(&mut rep).expect("Not a vaid answer");
    let rep:i32 = rep.trim().parse().expect("Not a valid answer");
    if rep == 1{
        println!("What level are you?");
        io::stdin().read_line(&mut level).expect("Not a valid input");
        let level:i32 = level.trim().parse().expect("Not a valid input");
        if level > 100{
            println!("What is your CGPA?");
            io::stdin().read_line(&mut gpa).expect("Not a valid number");
            let gpa:f32 = gpa.trim().parse().expect("Not a valid number");
            if gpa > 4.0{
                println!("Name: {} \nEmail: {} \nDepartment: {} \nState of Origin: {} \nYou can vote.", name,email,
            department,state);
            }
            else{
                println!("Sorry you are not eligible to vote.");
            }
        }else{
                println!("Sorry you are not eligible to vote.");
            }
    }else{
        println!("Sorry you are not eligible to vote");
    }
}
}