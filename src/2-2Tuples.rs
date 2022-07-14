use std::fmt::{Display,Formatter,Result};

fn reverse( pair : (i32, bool)) -> (bool, i32) {
    let (a , b) = pair;
    return (b , a);
}

fn transpose( mat : Matrix ) -> Matrix {
    return Matrix(
        mat.0, mat.2, mat.1, mat.3
    );
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(f, "({} {}) \n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    println!("{}", long_tuple.0);
    println!("{}", long_tuple.1);

    let triple_tuple = (1u8, 2u16, 3u32);

    println!("{:?}", triple_tuple);
    // println!("{:?}", long_tuple); // cannot print long tuple

    let pair = (1i32,true);

    println!("{:?}", reverse(pair));

    let (a,b,c) = (1,2,3);
    println!("{} {} {}", a, b, c);

    let matrix = Matrix(1.0,2.0,3.0,4.0);
    println!("{:?}",matrix);
    println!("original");
    println!("{}", matrix);
    println!("transpose");
    println!("{}",transpose(matrix));
}
