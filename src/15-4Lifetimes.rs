// Lifetimes is a construct the compiler uses to ensure all borrows are valid
// lifetime and scope are not same (Think borrow variable via `&`)


// Explict annotation
// How long references should be valid (when lifetime is implicitly annotated)
// Similar to closure, Lifetimes requires generic
fn explicit_annotation(){
    // 'a, 'b at least as long as 'print_refs()'
    fn print_refs<'a,'b>(x : &'a i32, y : &'b i32){
        println!("x : {}, y : {}", x, y);
    }
    fn failed_borrow<'a>(){
        let _x = 12;
        // let y: &'a i32 = &_x; // '_x' does not live long enough (short lifetime cannot be coerced into a longer one)
    }
    let (four, nine) = (4,9);
    print_refs(&four, &nine);
    failed_borrow();
}

// Functions
// Ignoring ellision, Function signatures with lifetimes have a few constraints
// - any reference must have an annotated lifetime
// - any reference being returned must have the same lifetime as an input or be static
// - returning references without input is banned if it would result in returning references to invalid data
fn functions(){
    fn print_one<'a>(x: &'a i32){ // 'a live as long as the function
        println!("print_one : x is {}",x);
    }
    fn add_one<'a>(x: &'a mut i32){ // 'a live as long as the function
        *x += 1;
    }
    fn print_multi<'a,'b>(x : &'a i32, y: &'b i32){ // in this case, we can only use 'a, but in more complex case, different lifetime may be required
        println!("print_multi : x is {}, y is {}",x,y);
    }
    fn pass_x<'a,'b>(x: &'a i32, _ : &'b i32) -> &'a i32{ // correct lifetime must be returned
        x 
    }
    // 'a must live longer than function but '&String::from("invalid")' is dropped upon existing the scope
    // fn invalid_output<'a>()->&'a String{ &String::from("invalid") } // Error (returns a reference to data owned by the current function

    let x = 7;
    let y = 10;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(&z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}


fn main() {
    explicit_annotation();
    functions();
}
