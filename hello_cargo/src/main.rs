extern crate rand;
use std::io;


use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1,100);

    println!("the secret number is::{}", secret_number);
    println!("this is a guessing game");
    let mut guess = String::new();

    let guess:u32 = guess.trim().parser().expect("error"); 

    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("guess value is ::{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small");
        Ordering::Greater => println!("too big");
        Ordering::Equal => println!("thats correct");
    }
}
