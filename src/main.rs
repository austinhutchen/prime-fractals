use core::rand::rng;

fn prime() -> u8 {
    let mut rng = rand::rng();
    let n2: u8 = rng.gen_range(0..1500);

    n2 = rng.gen();
    for x in 2..(n2 / 2) {
        if n2 % x == 0 {
            return prime();
        }
    }
    // n2 is prime after making it through all numbers previous on [2,n2/2]
    return n2;
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
