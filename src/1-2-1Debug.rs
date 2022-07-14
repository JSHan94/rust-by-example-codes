#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a>{
    name : &'a str,
    age : u8
}

fn main() {    
    let tmp = Structure(10);
    println!("{:?}", tmp);
    println!("{:?}", Deep(tmp));

    let peter = Person{
        name : "Peter",
        age : 3
    };

    println!("{:#?}", peter); // impl fmt::Display for custom formatting
}
