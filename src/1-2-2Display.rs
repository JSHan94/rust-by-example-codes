use std::fmt::{Display,Result,Formatter};

#[derive(Debug)]
struct MinMax(i32,i32);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f, "({} {})",self.0, self.1)
    }
}

#[derive(Debug)]
struct Point{
    x : i32,
    y : i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) ->Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {    
    let minmax = MinMax(1,10);

    println!("Debug MinMax : {:?}",minmax);
    println!("Display MinMax : {}",minmax);

    let point = Point{
        x : 10,
        y : 20,
    };

    println!("Debug Point : {:?}", point);
    println!("Display Point : {}", point);
}
