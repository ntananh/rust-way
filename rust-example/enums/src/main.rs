// Create an `enum` to classify a web event. Note how both 
// name and type information together specify the variant:
// `PageLoad != PageUnLoad` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`
    PageLoad,
    PageUnLoad,
    // like typle structs
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click{ x:i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"), 
        WebEvent::PageUnLoad => println!("page unloaded"), 
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c), 
        WebEvent::Paste(s) => println!("pasted \"{}\".", s), 
        // Destructure `Click` into `x` and `y`
        WebEvent::Click { x , y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates and owned `String` from a string slice
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load   = WebEvent::PageLoad;
    let unloaded   = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unloaded);
}
