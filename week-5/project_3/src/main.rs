// program to display menu and take order

use std::io;

fn main(){
    let m = "Menu                             Price".to_string();
    let  p = "1 = Poundo Yam/Edinkaiko Soup      -N3,200".to_string();
    let  f = "2 = Fried Rice & Chicken           -N3,000".to_string();
    let  a = "3 = Amala & Ewedu Soup             -N2,500".to_string();
    let  e = "4 = Eba & Egusi Soup               -N2,000".to_string();
    let  w = "5 = White Rice & Stew              -N2,500".to_string();

    let me = format!("{} \n {} \n {} \n {} \n {} \n {}", m,p,f,a,e,w);
    println!("{}",me );


    println!("Enter your order?");
     let mut order = String::new();
     io::stdin().read_line(&mut order).expect("Not a valid order");
     let order:i32 = order.trim().parse().expect("Not a valid order");

     println!("How many quantities?");
     let mut quantity = String::new();
     io::stdin().read_line(&mut quantity).expect("Not a valid quantity");
     let quantity:i32 = quantity.trim().parse().expect("Not a valid quantity");

     let mut m:i32 = 0;
     let d:i32 = 10_000;
     let j:i32;

    if order == 1
    {
      let m:i32 = quantity * 3_200;
      println!("Your order total is N{}.", m);
    }
    else if order == 2
    {
        let m:i32 = quantity * 3_000;
     println!("Your order total is N{}.", m);
    }
    else if order == 3
    {
       let m:i32 = quantity * 2_500; 
     println!("Your order total is N{}.", m);
    }
     else if order == 4
     {
        let m:i32 = quantity * 2_000; 
     println!("Your order total is N{}.", m);
     }
     else if order == 5
      {
        let m:i32 = quantity * 2_500;
        println!("Your order total is N{}.", m);
    }
    else 
    {
      println!("Enter your order");
    }
  
   
    if m > d {
      j = m - ((5/100) * m);
    println!("Yayyy!!, Your total is now: N{}.",j );
   }

    
     



}

    