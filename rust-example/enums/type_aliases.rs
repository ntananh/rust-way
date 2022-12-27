enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

// Create a type aliases
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) ->  i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}


fn main() {
    // We can refer to each variant via its alias, not  its long and inconvinient name.
    let x = Operations::Add;
}

