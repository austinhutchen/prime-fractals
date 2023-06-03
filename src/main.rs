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
fn main() {
    println!("< WELCOME TO ART GENERATOR 3000!");
    let limit = primegen();
    for x in 1..limit {
        println!("");
        if isprime(x) {
            print!("✪");
            // prime numbers only
            for z in 0..primegen() {
                print!("{z}");
                for y in 0..z {
                    print!(" ")
                }
            }
        } else {
            // not prime
            printdivisors(x);
            println!(" ");
        }
    }

    println!("");
    println!("DONE");
    return;
}
 