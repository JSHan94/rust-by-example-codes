fn main() {
    let first;
    {
        let x = 2;
        first = x * x;
    }
    println!("first {}", first);
    
    let second;

    // println!("{}",second); // error
}
