mod components;

fn main() {
    println!("< WELCOME TO PRIME ART GENERATOR 3000!");
    let limit = components::primegen();
    let mut i: usize = 0;
    while i < 3 {
        for x in 1..limit {
            println!("");
            if components::isprime(x) {
                print!("✪");
                components::shapegen(x);
                // prime numbers only
            } else {
                // not prime
                components::printdivisors(x);
                println!(" ");
            }
        }
        i = i + 1;
    }
}
