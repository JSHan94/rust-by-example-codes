fn main() {
    let an_int = 1u32;
    let a_boolean = true;
    let unit = ();
    
    let copied_int = an_int;

    println!("int {}", copied_int);
    println!("bool {}", a_boolean);
    println!("unit {:?}",unit);

    let _unused_var = 1u32; // no warning
    let unused_var = 2u32;  // warning
}
