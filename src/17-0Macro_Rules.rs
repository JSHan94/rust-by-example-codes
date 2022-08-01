// Macro system allows metaprogramming ()
// Rust macros are expanded into abstract syntax trees, rather than string preprocessing (Don't get unexpected precedence bugs)

macro_rules! say_hello{
    () => {
        println!("Hello world");
    };
}
fn main(){
    say_hello!();
}