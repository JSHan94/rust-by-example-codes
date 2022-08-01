// Designator : arguments of a macro
// available designators
// - block, expr (expression), ident (variable/function name), item, literal (literal constants)
// - pat (pattern), path, stmt (statement), tt (token tree), ty (type), vis (visibility qualifiers)
macro_rules! create_function{
    ($func_name : ident) => {
        fn $func_name(){
            println!("you called {:?}()", stringify!($func_name));
        }
    }
}
macro_rules! print_result{
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    }
}
create_function!(foo);
create_function!(bar);

// Overload
macro_rules! test{
    ($left : expr; and $right : expr) => { // semicolon and comma can be used
        println!("{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right)
    };
    
    ($left : expr; or $right : expr) => {
        println!("{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right)
    };
}

// Repeat
// Macro can use `+` and `*` for repeat
macro_rules! find_min{
    ($x:expr) => ($x);
    ($x:expr, $($y:expr), +) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main(){
    // Designator
    println!("1.Designator");
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    // Overload
    println!("2. Overload");    
    test!(1i32 + 1 == 2i32;  and 2i32 * 2 == 3i32);
    test!(true ; or false);

    // Repeat
    println!("3. Repeat");
    println!("{}", find_min!(1));
    println!("{}", find_min!(1+2,2));
    println!("{}", find_min!(5,2*3 + 5));
}