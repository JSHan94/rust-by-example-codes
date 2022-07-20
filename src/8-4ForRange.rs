fn main() {
    for n in 0..10{
        println!("0..10 : {}", n);
    }  
    
    for n in 0..=10{
        println!("0..=10 : {}", n);
    }

    // into_iter, iter, iter_mut
    let names = vec!["AA", "BB", "CC"];
    for name in names.iter(){
        match name {
            &"AA" => println!("Found &AA in names.iter()"),
            _ => (),
        }
    }
    
    let names = vec!["AA", "BB", "CC"];
    for name in names.into_iter(){
        match name{
            "AA" => println!("Found AA in names.into_iter()"),
            _ => ()
        }
    }

    let mut names = vec!["AA", "BB", "CC"];
    for name in names.iter_mut(){
        match name{
            &mut "AA" => println!("Found &mut AA in names.iter_mut()"),
            _ => ()
        }
    }
}
