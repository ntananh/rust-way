use std::mem;
// an array is a collection of object of the same type `T`, stored in contiguous memory.    

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]); 
    println!("the slice has {} element", slice.len()); 
}

fn main() {
    // fixed-size array (type signature is  superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
    // all elements can be initialized to the same value
    // value of all elements is 50
    let ys: [i32; 500] = [50; 500];

    // indexing start at 0
    println!("first element of the array: {}", xs[0]); 
    println!("second element of the slice: {}", xs[1]); 
 
    // `len` returns the count of elements in the array
    println!("first element of the array: {}", xs.len()); 

    
    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    analyze_slice(&xs);

    // slices can point to a section of an array
    // they are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice 
    println!("\nborrow a section of the array as a slice");
    analyze_slice(&ys[2..4]);

    // example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // arrays can be safety accessed using `.get`, which returns an
    // `Option`. This can be match as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happy continue.
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval)  => println!("{}: {}", i, xval),
            None => println!("slow down! index: {} is  to far!", i),
        }
    }

    // out of bound indexing causes runtime error
    // println!("{}", xs[5]);

}

