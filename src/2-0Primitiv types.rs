fn main() {
    let logical : bool = true;
    
    let a_float: f64 = 1.0;
    let an_integer = 6i32; 

    let default_float = 3.0; // f64
    let default_int = 3; // i32

    let mut inferred_type = 12; // i64 because of context (see below)
    inferred_type = 4294967296i64;

    let mut mutable = 12; // i32
    mutable = 21;

    // mutable = true  // error!

    let mutable = true; // shadowing
}
