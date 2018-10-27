extern crate algorithm;

pub use algorithm::sort::*;

fn main() { 
    let mut primes: Vec<u32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let len = primes.len()-1;
    quick_sort(&mut primes, 0,  len);
    for i in &primes {     
        println!("{}", i);
    }
}

