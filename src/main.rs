extern crate rand;
use rand::Rng;

use std::io;

fn main() {
    let p_number:u8 = rand::thread_rng().gen_range(1,7);
    println!("Type some number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u8 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => 1
    };

    if p_number == n {
        println!("You died!");
    } else {
        println!("You are alive!");
    }
}