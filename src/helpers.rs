use std::usize;

use rand::{self, Rng};

pub fn primegen() -> usize {
    let rand: usize = rand::thread_rng().gen_range(2..1500);
    // sieve method for small prime gaps.. for larger sampling sizes more complex gap analysis is needed
    for x in 2..(rand / 2) {
        if rand % x == 0 {
            return primegen();
        }
    }
    // n2 is prime after making it through all numbers previous on [2,n2/2]
    return rand;
}
pub fn isprime(i: usize) -> bool {
    for x in 2..(i / 2) {
        if i % x == 0 {
            return false;
        }
    }

    return true;
}
pub fn printdivisors(i: usize) {
    for x in 2..(i / 2) {
        if i % x == 0 {
            print!("{x} ,");
        }
    }
}
pub fn fractal(max:usize){
    for z in 0..max {
        let rand = max % primegen();
        let mut y: usize = 0;
        while y < z {
            print!(" ");
            y = y + 1;
        }
        print!("{rand}");
    }

}
pub fn shapegen(mut max: usize) {
    let bound: usize = 3000;
    let  rand: usize = rand::thread_rng().gen_range(2..bound);
    max = max << 1;
    if rand < bound / 2 {
        // circle
        
    } else {
        // prints a fractal output to command line with size max
       fractal(max);
    }
}
