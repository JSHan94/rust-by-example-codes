// `crate_type` : whether a crate is binary or library
// `crate_name` : set name of crate
// but they have no effect when using Cargo

#![crate_type = "lib"]
#![crate_name = "rary"]

pub fn public_function(){
    println!("called rary's public function");
}
fn private_function(){
    println!("called rary's private function");
}
pub fn indirect_function(){
    print!("called rary's indirect function > ");
    private_function();
}

// $rustc 13-2Crates.rs
// $ls lib*