use std::usize;

use rand::{self, Rng};

fn primegen() -> usize {
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
fn isprime(i: usize) -> bool {
    for x in 2..(i / 2) {
        if i % x == 0 {
            return false;
        }
    }

    return true;
}
fn printdivisors(i: usize) {
    for x in 2..(i / 2) {
        if i % x == 0 {
            print!("{x} ,");
        }
    }
}

fn shapegen(mut max: usize) {
    let bound: usize = 3000;
    let mut rand: usize = rand::thread_rng().gen_range(2..bound);
    max = max << 1;
    if rand < bound / 2 {
        // circle
        for z in 0..max {
            rand = max % primegen();
            let mut y: usize = 0;
            while y < z {
                print!(" ");
                y = y + 1;
            }
            print!("{rand}");
        }
    } else {
        // square
        for z in 0..max {
            rand = max % primegen();
            let mut y: usize = 0;
            while y < z {
                print!(" ");
                y = y + 1;
            }
            print!("{rand}");
        }
    }
}
fn main() {
    println!("< WELCOME TO PRIME ART GENERATOR 3000!");
    let limit = primegen();
    let mut i: usize = 0;
    while i < 3 {
        for x in 1..limit {
            println!("");
            if isprime(x) {
                print!("✪");
                shapegen(x);
                // prime numbers only
            } else {
                // not prime
                printdivisors(x);
                println!(" ");
            }
        }
        i = i + 1;
    }
}
