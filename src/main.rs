
mod lib;

fn main() {
    println!("< WELCOME TO PRIME ART GENERATOR 3000!");
    let limit = lib::getprimes();
        for x in 1..limit.len() {
            let val :u64 = limit[x];
            println!("");
                // not prime
                print!("{val}");
                println!(" ");
                lib::shapegen(limit.len());
                lib::printdivisors(x);
            }
        
        
}
