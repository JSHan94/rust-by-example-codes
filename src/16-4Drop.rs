// `Drop` trait only have `drop` method
// `Box`, `Vec`, `String`, `File`, `Process` are some examples that implement the `Drop` trait

struct Droppable{
    name : &'static str
}
impl Drop for Droppable{
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main(){
    let _a = Droppable { name : "a" };
    {
        let _b = Droppable { name : "b" };
        {
            let _c = Droppable { name : "c" };
            let _d = Droppable { name : "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(_a); // can be manually dropped
    println!("End of the main Function");
} 