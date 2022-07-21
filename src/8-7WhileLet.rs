fn main() {
    let mut optional = Some(0);
    while let Some(x) = optional{
        println!("current optional: {:?}", Some(x));
        if x > 10 {
            break;
        }
        optional = Some(x+1);
    }
}
