fn main() {
    let short_live =1;
    {
        let short_live =2;
        println!("short : {}",short_live);
    }
    println!("short : {}",short_live); // shadowing
}
