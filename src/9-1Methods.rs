#[derive(Debug)]
struct Point {
    x : f64,
    y : f64,
}

impl Point {
   fn original() -> Point {
       Point { x : 0.0, y : 0.0}
   }
   fn new(x : f64, y : f64) -> Point {
       Point {x:x, y:y}
   }
}

#[derive(Debug)]
struct Rectangle {
    p1 : Point,
    p2 : Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1-x2) * (y1-y2)).abs()
    }

    fn translate(&mut self, x : f64, y : f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }

    fn destroy(self){
        let Rectangle{ p1 : point1, p2 : point2 } = self;
        println!("{:?}, {:?} are destroyed", point1, point2);
    }
}

fn main() {
    let mut rectangle = Rectangle{
        p1 : Point::original(),
        p2 : Point::new(3.0, 4.0),
    };

    println!("before translate : {:?}",rectangle);
    rectangle.translate(5.0, 6.6);
    println!("after translate : {:?}",rectangle);
    rectangle.destroy();

}
