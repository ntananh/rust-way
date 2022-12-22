/*
There are 3 types of structure("structs") that can be created using the `struct` keyword:
    - tuple structs, which are, basically, named tuples.
    - the classic C struct
    - Unit structs, which are field-less, are useful for generics.
*/

// an attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// a unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f32);

// a tuple struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

// struct can be reused as field of another struct
#[derive(Debug)]
struct Rectangle {
    // a rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rect: &Rectangle) -> f32{
    let _width = rect.top_left.x + rect.top_left.y;
    let _height = rect.bottom_right.x + rect.bottom_right.y;
    println!("The area of {:?} and {:?} is: 10", rect.top_left, rect.bottom_right );
    _width * _height
}

fn main() {
    // create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // print debug struct
    println!("{:?}", peter);

    // instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // make a new point by using struct updat syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point};
    
    // `bottom_right.y` will be the same as `point.y` because we use the fields of our 
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);


    // destructure the point using a `let` binding
    let Point { x: let_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: let_edge, y: top_edge },
        bottom_right
    };

    // instantiate a unit struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    
    // destructure a tuple struct
    let Pair( integer, decimal ) = pair;
    println!("Destructure pair: pair Constants {:?} and {:?}", integer, decimal);


    // TODO calculate rect area
    rect_area(&_rectangle);

    println!(" Rectangle is {:?}", _rectangle);
}
