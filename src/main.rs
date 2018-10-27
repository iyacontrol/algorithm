

fn main() {
    
    let mut primes: Vec<u32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let len = primes.len()-1;
    quick_sort(&mut primes, 0,  len);
    for i in &primes {     
        println!("{}", i);
    }
}

fn quick_sort(vec : &mut Vec<u32>, low: usize, high: usize ) {
    if low < high {
       let mut i  = low;
       let mut j  = high;
       let base = vec[low];

       loop { 
           while vec[j] >= base && i < j  {
               j -=  1 ;
           }
           while vec[i] <= base && i < j {
               i += 1;
           }
           if i >= j {
               break;
           }
           vec.swap(i, j);
       }

       vec.swap(low, i);
       if i >= 1 {
            quick_sort(vec, low, i-1);
       } 
       quick_sort(vec, i+1, high);
    }
    
}
