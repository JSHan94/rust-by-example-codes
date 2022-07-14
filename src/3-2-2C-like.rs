enum Number{
    Zero,
    One,
    Two
}

enum Color{
    Red = 55,
    Green = 66,
    Blue = 77
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);  
    
    println!("red is {}", Color::Red as i32);
}
