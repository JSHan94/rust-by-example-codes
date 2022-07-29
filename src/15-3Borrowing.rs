// Borrowing : Access data without taking ownership
fn eat_box_i32(boxed_i32: Box<i32>){
    println!("Destroying box that contains {}", boxed_i32);
}
fn borrow_i32(borrow_i32: &i32){
    println!("borrow_i32 is {}", borrow_i32);
}
fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);

    mutability();
    aliasing();
    refpattern();
}


// Mutability
#[allow(dead_code)]
#[derive(Clone,Copy)]
struct Book{
    author : &'static str, // reference to a string allocated in read only memory
    title : &'static str,
    year : u32
}
fn borrow_book(book : &Book) {
    println!("immutably borrowed {} - {}", book.title, book.year);
}
fn new_edition(book : &mut Book) {
    book.year = 2004;
    println!("mutably borrowed {} - {}", book.title, book.year);
}
fn mutability(){
    let immut_book = Book {
        author : "Hood Lobin",
        title : "Harvey",
        year : 3002
    };
    let mut mut_book = immut_book; // mutable copy of immut_book
    borrow_book(&immut_book);
    borrow_book(&mut_book);
    new_edition(&mut mut_book);
}

// Aliasing
// Data can be immutably borrowed any number of times, but the original data can't be mutably borrowed
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32, z: i32}
fn aliasing(){
    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let another_borrowed_point = &point;

    println!("point coords : {} {} {}", borrowed_point.x, borrowed_point.y, borrowed_point.z);
    // let mut_borrowed = &mut point; // Error!
    println!("point coords : {} {} {}", another_borrowed_point.x, another_borrowed_point.y, another_borrowed_point.z);
    let mut_borrowed = &mut point;
    mut_borrowed.x = 100;

    println!("mutable borrowd points : {} {} {}",mut_borrowed.x, mut_borrowed.y, mut_borrowed.z);
}

// ref pattern

fn refpattern(){
    let c = 'Q';
    let ref ref_c1 = c;     // ref pattern 1
    let ref_c2 = &c;        // ref pattern 2
    println!("ref_c1 == ref_c2 : {}", ref_c1 == ref_c2);
    
    let point = Point {x :0, y:0 , z:0};
    let _copy_of_x = {
        let Point{ x: ref ref_to_x, y : _ , z : _} = point;
        *ref_to_x
    };
    let mut mutable_point = point;
    {
        let Point { x: _, y: ref mut mut_ref_to_y, z:_ } = mutable_point;
        *mut_ref_to_y = 1;
    }
    println!("point is ({}, {}, {})", point.x, point.y, point.z); // need Copy trait and Copy trait needs Clone trait
    println!("mutable point is ({}, {}, {})", mutable_point.x,mutable_point.y, mutable_point.z);

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}