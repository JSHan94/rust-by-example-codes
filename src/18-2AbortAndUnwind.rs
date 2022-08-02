// Different code path can be conditionally compiled based on the panic setting

fn drink(beverage : &str) {
    if beverage == "lemonade"{
        if cfg!(panic="abort"){ println!("This is not your party.."); }
        else{ println!("Spit it"); }
    }
    else { println!("Some refreshing {}", beverage); }
}

#[cfg(panic="unwind")]
fn ah(){ println!("Spit it out!!"); }
#[cfg(not(panic="unwind"))]
fn ah(){ println!("This is not your party.."); }

fn drink_unwind(beverage : &str) {
    if beverage == "lemonade"{ ah(); }
    else{ println!("Some refreshing {}",beverage); }
}

fn main(){
    drink("water");
    drink("lemonade");

    drink_unwind("water");
    drink_unwind("lemonade");
}

// rustc 18-2AbortAndUnwind.rs -C panic=abort && ./18-2AbortAndUnwind