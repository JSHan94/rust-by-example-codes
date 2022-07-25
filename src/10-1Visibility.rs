mod my_mod{
    fn private_function(){
        println!("Called `my_mod::private_function()`");
    }
    pub fn function(){
        println!("Called `my_mod::function()`");
    }
    pub fn indirect_function(){
        println!("Called `my_mod::indirect_function()`");
        println!("Called `my_mod::private_function()`");
    }

    pub mod nested{
        pub fn function(){
            println!("Called `my_mod::nested::function()`");
        }
        fn private_function(){
            println!("Called `my_mod::nested::private_function()`");
        }
        // `pub(in path)` syntax only visible in given path
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            println!("Called `my_mod::nested::public_function_in_my_mod");
        }
        // `pub(self)` syntax only visible within current module
        pub(self) fn public_function_in_nested(){
            println!("Called `my_mod::nested::public_function_in_nested`");
        }
        // `pub(super)` only visible whinin parent module
        pub(super) fn public_function_in_super_mod(){
            println!("Called `my_mod::nested::public_function_in_super_mod`");
        }
    }

    pub fn call_public_function_in_my_mod(){
        println!(">>>");
        println!("called `my_mod::call_public_function_in_my_mod`");
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
        println!(">>>");
    }

    // `pub(crate)` only visible within current crate
    pub(crate) fn public_function_in_crate(){
        println!("Called `my_mod::public_function_in_crate`");
    }

    mod private_nested{
        pub fn function(){
            println!("Called `my_mod::private_nested::function`");
        }
        pub(crate) fn restricted_function(){
            println!("Called `my_mod::private_nested::restricted_function`");
        }
    }
}
fn function(){
    println!("Called `function()`");
}
fn main() {
    function();
    my_mod::function();
    my_mod::nested::function();

    my_mod::indirect_function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();


    // Errors
    // my_mod::nested::public_function_in_my_mod();
    // my_mod::private_function();
    // my_mod::nested::private_function();
    // my_mod::private_nested::function();
    // my_mod::private_nested::restricted_function();
}
