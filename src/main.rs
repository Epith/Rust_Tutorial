use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    //match with cmp
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };

    //match statement
    // let age2 = 8;
    // match age2 {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("Important Birthday"),
    //     65..=i32::MAX => println!("Important Birthday"),
    //     _ => println!("Not an important Birthday"),
    // };
    // let age = 8;
    // if (age >= 1) && (age <= 18) {
    //     println!("Important Birthday");
    // } else if (age == 21) || (age == 50){
    //     println!("Important Birthday");
    // } else if age >= 65 {
    //     println!("Important Birthday");
    // } else {
    //     println!("Not an important birthday");
    // }

    //ternary operator
    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 {
    //     true
    // } else{
    //     false
    // };
    // println!("Can Vote: {}", can_vote)
    
}
