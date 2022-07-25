// hierarchy
// .
// |-- my
// |   |-- inaccessible.rs
// |   |-- mod.rs
// |   `-- nested.rs
// `-- split.rs

// split.rs
// mod my;
// fn function() {
//     println!("called `function()`");
// }
// fn main() {
//     my::function();
//     function();
//     my::indirect_access();
//     my::nested::function();
// }

// my/mod.rs
// mod inaccessible;
// pub mod nested;
// pub fn function() {
//     println!("called `my::function()`");
// }
// fn private_function() {
//     println!("called `my::private_function()`");
// }
// pub fn indirect_access() {
//     print!("called `my::indirect_access()`, that\n> ");
//     private_function();
// }

// $ rustc split.rs && ./split
// called `my::function()`
// called `function()`
// called `my::indirect_access()`, that
// > called `my::private_function()`
// called `my::nested::function()`


fn main() {

}
