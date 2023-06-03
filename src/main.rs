use rand::{self, Rng};


fn prime() -> usize {
 let mut rand = rand::thread_rng().gen::<usize>();
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
    for x in 1..bound {
        println!("");
        for z in 0..x {
            for y in 0..z {
                print!(" ")
            }
            print!("*");
        }
    }

    println!("");
    println!("DONE");
    return;
}
