fn main() {
    rary::public_function();
    rary::indirect_function();
}


// rustc 11-2UsingLibrary.rs --extern rary=lib11_1CreatingLibrary.rlib  &&./11-2UsingLibrary