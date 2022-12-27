use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Ni: A node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create en empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type list
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method 
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a 
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) bellow as well
        // rust will infe &s and ref tail.

        match *self {
            // Can't take ownership of the tail, because `self` is borrowed; 
            // instead take a reference to the tail
            Cons(_, ref tail) =>  1 + tail.len() ,
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref  tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated stirng instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create and empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(3);
    list = list.prepend(2);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

