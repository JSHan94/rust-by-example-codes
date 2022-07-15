fn main() {
    // type inference looks how it is used
    
    let elem = 5u8;

    // compiler doesn't know type yet (Vec<_>)
    let mut vec = Vec::new();
    
    // compiler know type (Vec<u8>)
    vec.push(elem);

    println!("{:?}", vec);
}
