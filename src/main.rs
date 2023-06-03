use rand::{self, Rng};

fn prime() -> usize {
     let rand:usize = rand::thread_rng().gen_range(0..1000);
    // sieve method for small prime gaps.. for larger sampling sizes more complex gap analysis is needed
    for x in 2..(rand / 2) {
        if rand % x == 0 {
            return prime();
        }
    }
    // n2 is prime after making it through all numbers previous on [2,n2/2]
    return rand;
}

fn main() {
    println!("----F(X) LOADING----");
    println!("< WELCOME TO ART GENERATOR 3000!");

    let bound = prime();
    for x in 1..prime() {
        println!("");
        for z in 0..prime() {
            for y in 0..prime() {
                print!(" ")
            }
            print!("*");
        }
    }

    println!("");
    println!("DONE");
    return;
}
