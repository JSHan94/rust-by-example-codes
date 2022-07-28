// To stipulate what functionality a type implements, use trait as `bounds`

// make `T` to be bound by `Display` trait
use std::fmt::Display;
fn printer<T: Display>(t:T){
    println!("{}", t);
}

// generic type that conform bounds
// struct S<T:Display>(T);
// let s = S(vec![1,2,3]); // Error, Vec<T> does not implement `Display`

use std::fmt::Debug;

trait HasArea{
    fn area(&self) -> f64;
}
impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.width }
}

// empty bounds
trait EmptyBounds {}
impl EmptyBounds for Rectangle {}
fn empty_bounds<T:EmptyBounds>(_: &T) -> &'static str {"empty bounds"}

#[derive(Debug)]
struct Rectangle { length : f64, width : f64}

fn print_debug<T: Debug>(t : &T){
    println!("{:?}", t);
}

fn area<T: HasArea>(t : &T) -> f64 { t.area()}
fn main() {
    let rect = Rectangle{length : 5.0, width : 5.0};
    print_debug(&rect);
    println!("{}", area(&rect));

    println!("{}", empty_bounds(&rect));
} 
