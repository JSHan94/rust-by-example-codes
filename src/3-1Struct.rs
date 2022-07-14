use std::fmt;
#[derive(Debug)]
struct People{
    name :String,
    age : u32
}

struct Unit;
struct Pair(i32, i32);
 
#[derive(Debug)]
struct Point(i32,i32);
#[derive(Debug)]
struct Rect(Point, Point);


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({} {})", self.0, self.1)
    }
}
impl fmt::Display for Rect{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "topleft: {}, downright: {}", self.0, self.1)
    }
}

fn rect_area(rect : Rect) -> i32 {
    let (topleft, downright) = (rect.0, rect.1);
    (downright.0 - topleft.0) * (topleft.1 - downright.1)
}

fn square(p : Point, width : f32) -> Rect {
    let another_p = Point(p.0+width as i32, p.1+width as i32);
    let topleft = Point(p.0, another_p.1);
    let downright = Point(another_p.0,p.1);
    Rect(topleft, downright)
}

fn main() {
    let topleft : Point = Point(6,2);
    let downright : Point = Point(3,4);
    
    let rect : Rect = Rect(topleft, downright);
    println!("{}",rect);
    println!("area : {}",rect_area(rect));
    println!("square : {} ", square(Point(0,0),3.0));
}
