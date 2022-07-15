// From과 Into는 본질적으로 연결되음
// 만약 A타입을 B로 변경하는 것이 가능하다면 B타입을 A타입으로 변경하는 것도 가능해야함

use std::convert::From;

#[derive(Debug)]
struct Number{
    value : i32,
}

// from은 다른 타입에서 해당 타입으로 바뀌는 방법을 정의함
// into는 From trait과 상호적이라 from trait을 구현하였다면 into 호출도 가능해야함
impl From<i32> for Number{
    fn from(item : i32) -> Self{
        Number { value : item}
    }
}


fn main() {
    let num = Number::from(30);
    println!("from called : {:?}", num);

    let num : Number = 5.into();
    println!("into called : {:?}", num);
}
