// String으로 타입을 바꾸는 가장 쉬운 방법은 ToString trait임
// ToString을 자동적으로 제공하기 위해서는 fmt::Display trait을 구현해야함 

use std::fmt;

struct Circle {
    radius : i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle{ radius: 9};
    println!("{}", circle.to_string());

    // string을 number로 바꾸는 과정 또한 빈번 함
    // 이상적인 방법은 parse 함수를 이용하는 것이고 type inference를 설정하거나 type을 turbofish syntax를 이용하여 명시하는 것임
    // 각각의 방법은 아래와 같음 (FromStr trait이 해당 타입에 구현되어있을 때만)    
    let parsed : i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap(); // turbofish

    let sum = parsed + turbo_parsed;
    println!("{:?}", sum);

}
