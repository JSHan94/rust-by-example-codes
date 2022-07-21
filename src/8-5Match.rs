fn main() {
    // basic
    let number = 13;
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 9 | 11 => println!("2 | 3 | 5 | 7 | 9 | 11"),
        13..=19 => println!("13..= 19"), // 13..19 is not allowed 
        _ => () // necessary to cover all possible cases
    }

    // tuples
    let triple = (0,-2,3);
    match triple {
        (0,y,z) => println!("(0,y,z)"), // cannot cover (i32::MIN .. i32::MAX, y, z)
        (1, .. ) => println!("(1, .. )"),
        _ => () 
    }

    // array
    let array = vec![[1,-2,6], [0, 3, 4], [-1, 3, 77], [3, 6, 22], [-4, 05, 7]];
    for arr in array.iter(){
        match arr {
            [0, y, z] => println!("{:?} is [0, y, z]", arr),
            [1, _, z] => println!("{:?} is [1, _, z]", arr),
            [-1, y, ..] => println!("{:?} is [-1, y, ..]", arr),
            [3, y, tail @..] => println!("{:?} is [3, y, tail @..", arr), // @ operator set range and bind variable
            [x, y @.., z] => println!("{:?} is [x, y @.., z]",arr)
        }
    }

    // enum
    #[derive(Debug)]
    enum Color{
        RED,
        RGB(u32, u32, u32),
        CMYK(u32, u32, u32, u32)
    }

    let colors = vec![Color::RGB(3,4,5),Color::CMYK(5,6,7,7), Color::RED];
    for color in colors.iter(){
        match color{
            Color::RED => println!("{:?} is Color::RED", color),
            Color::RGB(a,b,c) => println!("{:?} is Color::RGB", color),
            Color::CMYK(a,b,c,d) => println!("{:?} is Color::CMYK", color),
            _ => println!("{:?} is _", color)
        }
    }
    
    // pointers/ref    
    let references = vec![&4, &(-1)];
    for reference in references.iter(){
        match reference{
            &val => println!("{:?} is &val", reference),
        }
        match *reference {
            val => println!("{:?} is *val", reference),
        }
    }

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("{:?} is ref mut",m);
        }
    }

    // structs
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32
    }

    let foo = Foo { x:(1,2), y: 3 };
    match foo {
        Foo {x : (1,b), y} => println!("{:?} is Foo{{x:(1,b), y}}", foo),
        Foo {x, ..} => println!("{:?} is Foo{{x, ..}}", foo),
    }

    // match guards (filtering)
    let pairs = vec![(2,-2), (2,2), (3,0)];
    for pair in pairs {
        match pair {
            (x,y) if x == y => println!("{:?} if x == y", pair),
            (x,y) if x + y ==0 => println!("{:?} if x + y == 0 ", pair),
            (x,_) if x % 2 == 1 => println!("{:?} if x % 2 == 1 ",pair),
            _ => ()
        }
    }

    // match binding
    fn age() -> u32{
        15
    }
    match age() {
        0 => println!("not binding"),
        n @ 1..=3 => println!("n @ 1..=3"),
        n @ 3..=19 => println!("n @ 3..=19"),
        n => println!("my age is {:?}", age()),
    }

    fn some_number() -> Option<u32>{
        Some(42)
    }
    match some_number(){
        Some(n@2..=52) => println!("Some({:?}) is Some(n@2..52)", some_number().unwrap()),
        Some(n) => println!("Some({:?})Some(n)", some_number().unwrap()),
        _ => ()
    }
    
}
