// Compiler should know function return type (to know space)
// But it's hard to custom types with traits
// Instead of returning a trait object directly, return a `Box` -> Reference has statically-known size
// `dyn` explicit whenever it allocates memory on the heap

struct Sheep{}
struct Cow{}

trait Animal{
    fn noise(&self) -> &'static str;
}
impl Animal for Sheep{
    fn noise(&self) -> &'static str{
        "sheep noise"
    }
}
impl Animal for Cow{
    fn noise(&self) -> &'static str{
        "cow noise"
    }
}
fn random_animal(random_number : f64) -> Box<dyn Animal>{ // we don't know which will be returned in compile time
    if random_number < 0.5{
        Box::new(Sheep{})
    }else{
        Box::new(Cow{})
    }
}

fn main(){
    let random_number = 0.235;
    let animal = random_animal(random_number);
    println!("animal says : {}", animal.noise());
} 