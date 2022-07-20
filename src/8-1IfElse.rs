fn main() {
    let n = 5;
    
    if n < 0 {
        println!("{} is negative", n);
    } else{
        println!("{} is positive or zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!("{} is between 10 and -10",n);
            n * 10
        }else{
            println!("{} is not between 10 and -10",n);
            n / 2
        }; // all 'let' binding needs semicolon
    
    println!("big n : {}", big_n);
}
