fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // check where they are used
    let i = 1;
    let f = 1.0;

    println!("size of x : {}", std::mem::size_of_val(&x));
    println!("size of y : {}", std::mem::size_of_val(&y));
    println!("size of z : {}", std::mem::size_of_val(&z));
    println!("size of i : {}", std::mem::size_of_val(&i));
    println!("size of f : {}", std::mem::size_of_val(&f));
}
