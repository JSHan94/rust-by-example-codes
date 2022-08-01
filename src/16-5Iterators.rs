// `Iterator` trait used to implement iterators over collections
// only needs method for `next` element

struct Fibonacci{
    curr : u32,
    next : u32
}
impl Iterator for Fibonacci{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        // None never be returned
        Some(self.curr)
    }
}
fn fibonacci() -> Fibonacci{
    Fibonacci{ curr: 0, next: 1 }
}

fn main(){
    // take(n) reduces an `Iterator` to its first `n` terms
    for i in fibonacci().take(5){ 
        println!("fibonacci().take(5) > {} ", i);
    }
    for i in fibonacci().skip(5).take(5){
        println!("fibonacci().skip(5).take(5) > {}", i);
    }
}