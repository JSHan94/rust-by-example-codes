// Simplest error handling mechanism, Panic

fn drink(beverage : &str){
    if beverage == "lemonade" {
        panic!("nooo...");
    }
    println!("Some refreshing {}", beverage)
}
fn main(){
    drink("water");
    drink("lemonade");
}