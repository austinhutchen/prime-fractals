use std::usize;

use rand::{self, Rng};

fn prime() -> usize {
    let rand: usize = rand::thread_rng().gen_range(2..1500);
    // sieve method for small prime gaps.. for larger sampling sizes more complex gap analysis is needed
    for x in 2..(rand / 2) {
        if rand % x == 0 {
            return prime();
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
fn main() {
    println!("< WELCOME TO ART GENERATOR 3000!");

    for x in 1..prime() {
        println!("");
        if isprime(x) {
            for z in 0..x {
                print!("{x}");
                for y in 0..z {
                    print!(" ")
                }
            }
        }
    }

    println!("");
    println!("DONE");
    return;
}
