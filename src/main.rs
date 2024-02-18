use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assign a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    // use _ infront to let the compiler ignore it
    let _is_true = true;
    let _my_grade = 'A';
    println!("is this true {}", _is_true);
    println!("my grade is {}", _my_grade);
}
