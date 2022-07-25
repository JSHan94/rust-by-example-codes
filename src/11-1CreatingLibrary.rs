pub fn public_function(){
    println!("called rary's `public_function()`");
}
fn private_function(){
    println!("called rary's `private_function()`");
}
pub fn indirect_function(){
    println!("called rary's `indirect_function()`");
    private_function();
}
fn main() {
    public_function();
    println!("-----");
    private_function();
    println!("-----");
    indirect_function();
}
// make library
// rustc --crate-type=lib 11-1CreatingLibrary.rs