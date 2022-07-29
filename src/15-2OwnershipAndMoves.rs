// Resources can only have one owner (To prevent resources from being freed more than once)
// Not all variables own resources (e.g. `references`)
// Assignment and Passing function arguments transfer ownership (move)


fn destroy_box(c:Box<i32>){
    println!("Destroying a box that contains {}", c);
}

fn main() {
    let x = 5u32;   // _Stack_ allocated
    let y = x;      // Copy x into y (no resources are moved) 
    println!("x is {}, y is {}", x, y);

    let a = Box::new(3i32);
    let b = a;      // Move a int b
    destroy_box(b);

    let immutable_box = Box::new(5u32);
    println!("immutable_box is {}", immutable_box);
    let mut mutable_box = immutable_box;    // move the box
    *mutable_box = 4;
    println!("mutable_box is {}", mutable_box);


    #[derive(Debug)]
    struct Person{
        name : String,
        age : u8,
    }
    let person = Person{
        name : String::from("Alice"),
        age : 20,
    };
    let Person {name, ref age} = person;  // person.name is moved to age, person.age is not moved

    println!("name is {}", name);
    println!("age is {}", age);
}
