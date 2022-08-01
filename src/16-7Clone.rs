// `Clone` trait helps us to make a copy of the resources

#[derive(Debug,Clone,Copy)]
struct Unit;
#[derive(Clone,Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main(){
    let unit = Unit;
    let copied_unit = unit; // copy, no resources to move
    println!("original : {:?}", unit);
    println!("copy : {:?}", copied_unit);
    
    let pair = Pair(Box::new(11), Box::new(22));
    let moved_pair = pair;
    let cloned_pair = moved_pair.clone();
    // println!("orignal pair : {:?}", pair); // Error, Copy trait should be implemented!
    println!("moved pair : {:?}", moved_pair);
    drop(moved_pair);
    // println!("moved pair : {:?}", moved_pair); // Error
    println!("cloned pair : {:?}", cloned_pair);
}