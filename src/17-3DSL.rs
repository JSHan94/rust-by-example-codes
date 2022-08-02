// Domain Specific Language
// DSL define concise or intuitive syntax for special functionality (like mini 'language')

macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val : usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    }};
}

fn main(){
    calculate! {
        eval 1+ 2
    }
    calculate! {
        eval (1+2) * (3 / 4)
    }
}