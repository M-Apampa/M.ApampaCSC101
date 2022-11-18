//Rust program to find roots of quadratic equation

use std::io;

fn main() {
    println!("Enter the value of a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter the value of b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Failed to read input");

    println!("Enter the value of c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Failed to read input");

    let mut discriminant = (b * b) - (4.0 * a * c);
    let z:f32 = 2.0 * a;
    let root1:f32 = (-b + discriminant.sqrt() ) / z ;
    let root2:f32 = (-b - discriminant.sqrt() )/ z ;
    if discriminant>0.0 {
        println!("The roots of your quadratic equation are:{} and {}", root1 , root2 )
    }
    else if discriminant == 0.0 {
        println!("The roots of your quadratic equation are: {}", root1 );
    }
    else{
    println!("The quadratic equation has no roots.");
}

    
    
}
    


