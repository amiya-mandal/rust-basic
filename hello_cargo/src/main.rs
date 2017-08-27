extern crate rand;
use std::io;

use rand::Rng;


fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1,100);

    println!("the secret number is::{}", secret_number);
    println!("this is a guessing game");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("guess value is ::{}", guess);
}
