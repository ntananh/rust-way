/* Note:
 * Associate function do not take &self as argument
 * */


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { 

    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl  Rectangle {

    // associate function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 10
    };
    
    let larger_rect = Rectangle {
        width: 10,
        height: 10
    };

    let square = Rectangle::square(5);

    println!("The rectangle is {:#?}", rect);

    println!("The square is {:#?}", square);
        
    println!(
        "The area of the rectangle is {} square pixels",
        rect.area()
    );
    
    println!(
        "The rect {} hold the larger rect", 
         rect.can_hold(&larger_rect)
    );
}
