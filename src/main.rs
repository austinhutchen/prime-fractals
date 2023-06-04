mod helpers;

fn main() {
    println!("< WELCOME TO PRIME ART GENERATOR 3000!");
    let limit = helpers::primegen();
    let mut i: usize = 0;
    while i < 3 {
        for x in 1..limit {
            println!("");
            if helpers::isprime(x) {
                print!("✪");
                helpers::shapegen(x);
                // prime numbers only
            } else {
                // not prime
                helpers::printdivisors(x);
                println!(" ");
            }
        }
        i = i + 1;
    }
}
