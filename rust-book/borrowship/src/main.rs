fn main() {
    let mut s: String = String::from("foo");

    let r1 = &mut s;

    r1.push_str("hi");

    let r2 = &r1;

    println!("value get from reference 1: {}", r1); 

    println!("value get from reference 2: {}", r2); 
   
    //let s2: String = give_ownership();
    //let s3 = take_and_give_back(s2);

    //println!("take and give back = {}", s3);
}

fn give_ownership() -> String {
    println!("giving ownership...");
    let some_string: String = String::from("foo");
    some_string
}

fn take_and_give_back(a_string: String) -> String{
    a_string
}
