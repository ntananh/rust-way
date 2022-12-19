fn main() {
    // variable can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let a_integer = 5i32; // suffix annotation
    
    // or a default will be used
    let default_float = 3.0; // `f64`
    let default_integer = 3; // `i32`

    // a type can also be inferred from context
    let mut interred_type = 12; // type i64 is inferred from another line
    interred_type = 4294967296i64;

    // a mutable variable's value can changed.
    let mut mutable = 12; // mutable `12`
    mutable = 21;                      

    // Error! the type of a variable can't be changed.
    mutable = true;

    // variables can be overritten with shadowing.
    let mutable = true;
}
