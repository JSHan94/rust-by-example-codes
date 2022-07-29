// Variables in Rust do more than just hold data in stack
// e.g. Box<T> owns memory in heap
// Rust enforces RAII (Resource Acquisition Is Initialized)
// Destructor is called and owned resources are freed when object goes out of scope

fn create_box(){
    let _box1 = Box::new(3i32);
}


// `Drop` trait for destructor
struct ToDrop;
impl Drop for ToDrop{
    fn drop(&mut self){
        println!("ToDrop is being dropped")
    }
}

fn main() {
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000{
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop!");
}

// rustc 15-1RAII.rs && valgrind ./15-1RAII.rs
