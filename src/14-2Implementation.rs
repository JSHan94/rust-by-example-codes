struct S;
struct GenericVal<T>(T);

impl GenericVal<f32> {}
impl GenericVal<S> {}
impl<T> GenericVal<T> {} // <T> must precede the type remain generic

struct Val{
    val: f64,
}
struct GenVal<T> {
    gen_val : T,s
}
impl Val{
    fn value(&self) -> &f64 {
        &self.val
    }
}
impl <T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val {val: 0.0};
    let y = GenVal { gen_val: "generic" };

    println!("{} , {} ", x.value(), y.value());
}
