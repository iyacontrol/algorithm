mod sort;

use sort::*;

fn main() { 
    let mut primes: Vec<u32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let len = primes.len()-1;
    // quick_sort(&mut primes, 0,  len);
    bupple(&mut primes);
    for i in &primes {     
        println!("{}", i);
    }
}

