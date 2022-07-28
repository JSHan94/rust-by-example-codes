struct A; // concrete type
struct Single(A); // concrete type
struct SingleGen<T>(T); // generic type

fn main() {
    let _s = Single(A);
    let _char : SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
