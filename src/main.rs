
mod lib;

fn main() {
    println!("< WELCOME TO PRIME ART GENERATOR 3000!");
    let limit = lib::getprimes();
    let mut i: usize = 0;
        for x in 1..limit.len() {
            let val :u64 = limit[x];
            println!("");
                // not prime
                print!("{val}");
                println!(" ");
                lib::fractal(limit.len());
            }
            i = i + 1;
        
        
}
