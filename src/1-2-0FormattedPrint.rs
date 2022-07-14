fn main() {
    println!("{} days", 31);

    println!("{1} {0} {0} {1}", "zero", "one");

    println!("{sub} {verb} {ob}",
        sub = "sub",
        verb = "verb",
        ob = "ob");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);    // b stands for binary

    println!("{number:>width$}", number=1, width=6);

    println!("{number:>0width$}", number=1, width=6);

    println!("{pi:.*}",3,pi = 3.14159);

    /*
        // error codes 
        println!("My name is {0}, {}, {1}", "james");  // compile failed, error in {1}, not in {}
        #[allow(dead_code)] // disable lint
        struct Structure(i32);
        println!("{}", Structure(3)); // need std::fmt::Display trait
    */
}
