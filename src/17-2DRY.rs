// DRY : Don't Repeat Yourself
// Let's implement and test '*=', '+=', '-=' on Vec<T>

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len{
    // `tt` designator is used for operators and tokens
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!($a.len()==$b.len(),
        "{:?} dimension mismatch {:?} {:?} {:?}",
        stringify!($func),
        ($a.len(),),
        stringify!($op),
        ($b.len(),));
    }
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method: ident) => {
        fn $func<T: $bound<T,Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x,y) in xs.iter_mut().zip(ys.iter()){
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    }
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

fn main(){
    
}