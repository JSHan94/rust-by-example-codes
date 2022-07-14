fn main() {
    let _immutable = 1;
    let mut mutable = 1;

    println!("before {}", mutable);
    mutable += 1;
    println!("after {}", mutable);
    
    // _immutable +=1; // error
}
