enum Status{
    Rich,
    Poor
}

enum Work{
    Civilian,
    Soldier
}

fn main() {
    use crate::Status::*;
    use crate::Work::Civilian;
    
    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("rich"),
        Status::Poor => println!("poor")
    }

}
