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
    let contains = move |needle| haystack.contains(needle); // 'move ||' forces closure to take ownership of captured variables
    println!("{}", contains(&4));
    println!("{}", contains(&3));

    // Closure as input parameter
    // when taking a closure as an input parameter, closure's complete type must be annotated using trait.
    // compiler implicitly create a new anonymous structure to store the captured variables inside,
    // meanwhile implementing the functionality via one of traits
    // Fn : by reference (&T)
    // FnMut : by mutable reference (&mut T)
    // FnOnce : by value (T)
    fn apply<F>(f: F) where F: FnOnce(){
        f();
    }
    let mut farewell = "farewell with ".to_owned();
    let diary = || {
        farewell.push_str("closure");
        println!("{}",farewell);
        mem::drop(farewell);
    };
    apply(diary);
    
    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }
    let double = |x| x*2;
    println!("{}", apply_to_3(double));


    // function can be arguments too
    fn apply_to_function (){
        let mut farewell = "farewell with ".to_owned();
        farewell.push_str("function");
        println!("{}", farewell);
        mem::drop(farewell);
    }
    apply(apply_to_function);

    // returning closure as output is possible too
    fn create_fn()-> impl Fn(){
        let text = "Fn".to_owned();
        move || println!("This is a: {}",text)
    }

    fn create_fnmut()-> impl FnMut(){
        let text = "FnMut".to_owned();
        move || println!("This is a: {}",text)
    }

    fn create_fnonce()-> impl FnOnce(){
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}",text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();

    // iterator::any
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];
    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("is 2 in vec1[1]? : {}", vec1.clone().iter().any(|&x| x == 2 )); // iter() yields &i32, Destructure to i32
    println!("is 2 in vec1[1]? : {}", vec2.clone().into_iter().any(|x| x== 2)); // into_iter() yields i32, no Destructure
    println!("is 2 in array1[1]? : {}", array1.clone().iter().any(|&x| x == 2 )); 
    println!("is 2 in array2[1]? : {}", array2.clone().into_iter().any(|&x| x == 2 )); // array.into_iter() yields &i32
    
    // iterator::find
    println!("find 2 in vec1: {:?}", vec1.iter().find(|&&x| x==2));
    println!("find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x==2));
    println!("find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2 ));
    println!("find 2 in array2: {:?}", array2.iter().find(|&&x| x == 2 ));

    //iterator::position
    let index_of_two = vec1.iter().position(|x| x == &2);
    assert_eq!(index_of_two, Some(1)); 
}
