use std::mem;

fn analyze_slice(slice: &[u32]){
    println!("0 element : {}", slice[0]);
    println!("slice len : {}", slice.len() as usize);
}

fn main() {

    let xs : [i32 ; 5] = [1,2,3,4,5];
    let ys : [i32 ; 50] = [1 ; 50];

    println!("{}", xs[0]);
    println!("{}", xs.len() as usize);
    println!("array size : {}", mem::size_of_val(&ys));

    
    let slice = vec![1, 2, 3, 4];
    analyze_slice(&slice);
    analyze_slice(&slice[1..2]);
    // println!("{}", slice[100]); // out of bound error
}
