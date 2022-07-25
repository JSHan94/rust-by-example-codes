// Structs have an extra level of visibility (default is private)
mod my {
    pub struct OpenBox<T>{
        pub contents: T,
    }

    pub struct ClosedBox<T>{
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox{
                contents : contents,
            }
        }
    }

}
fn main() {
    let open_box = my::OpenBox { contents: "public information" };
    println!("my::OpenBox - {}", open_box.contents);

    
    let _closed_box = my::ClosedBox::new("closed information");
    
    // Errors
    // let closed_box = my::ClosedBox { contents: "closed information" };
    // println!("my::ClosedBox - {}", closed_box.contents);
    // println!("my::ClosedBox - {}", _closed_box.contents);
}
