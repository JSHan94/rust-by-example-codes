fn main() {
    let closure_annotated = | i : i32 | -> i32 {i+1};
    let closure_inferred = |i | {i+1};
    let one = ||{1};
    println!("clousure_annotated: {}", closure_annotated(10));
    println!("closure_inferred: {}", closure_inferred(10));
    println!("one: {}", one());

    // Clousure capturing
    // capture variables flexibly
    use std::mem;
    let color = String::from("green");
    let print = || println!("print something : {:?}", color);
    print();
    let _reborrow = &color;
    print();
    let _moved_color = color;

    let movable = Box::new(3); // non-copy type
    let consume = || {
        println!("movable : {:?}", movable); 
        mem::drop(movable); // it requires 'T' type. non-copy must move, 'movable' moves into clousure
    };
    consume();

    let haystack = vec![1,2,3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&4));
    println!("{}", contains(&3));

    // Closure as input parameter
    // when taking a closure as an input parameter, closure's complete type must be annotated using trait.
    // Fn : by reference (&T)
    // FnMut : by mutable reference (&mut T)
    // FnOnce : by value (T)
    fn apply<F>(f: F) where F: FnOnce(){
        f();
    }
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        farewell.push_str("!!");
        println!("{}",farewell);
        mem::drop(farewell);
    };
    apply(diary);
    
    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }
    let double = |x| x*2;
    println!("{}", apply_to_3(double));

}
