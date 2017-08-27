extern crate rand;
use std::io;


use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1,100); // this will genreate a random number

    println!("the secret number is::{}", secret_number); // this is display the secret number
    println!("this is a guessing game");
    loop {
        let mut guess = String::new();  // creating a guess var with String type

        io::stdin().read_line(&mut guess).expect("failed to read line"); // taking input form console

        // let guess:u32 = guess.trim().parse().expect("error"); // creating a "shadow var" of guess variable with u32 datatype

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("guess value is ::{}", guess);

        match guess.cmp(&secret_number) { // comparing both of the var 
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correct value ");
                break;
            }
            }
    }
}
