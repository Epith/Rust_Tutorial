use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}",val);
    }
    //arr_it.into_iter();
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}",iter1.next());
}
