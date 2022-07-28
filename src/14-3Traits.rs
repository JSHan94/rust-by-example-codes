// trait can also be generic

// Non-copyable types
struct Empty;
struct Null;

trait DoubleDrop<T> {
    // takes ownership
    fn double_drop(self, _:T);
}

impl<T,U> DoubleDrop<T> for U {
    // takes ownership of both passed arguments
    fn double_drop(self,_:T){}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    // empty;
    // null;
}


// trait : defining shared behavior
// impl : defining struct behavior