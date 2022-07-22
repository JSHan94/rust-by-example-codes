fn main() {
    fn foo() -> ! { // '!' is empty type
        panic!("panic never return"); 
    }
}
