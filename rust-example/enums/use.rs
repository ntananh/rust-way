#![allow(dead_code)]

enum Status {
    Rich, 
    Poor,
}

enum Work {
    Civilian, 
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are variable without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;
    
    // Equivalent to `Status::Poor`
    let status = Poor;
    // Equivalent to `Status::Civilian`
    let work = Civilian;


    match status {
        Rich => println!("The rich have a lots of money!"),
        Poor => println!("The rich have a lots of money!")
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Civilians fight!")
    }
}
