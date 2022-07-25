use deeply::nested::function as other_function;

fn function(){
    println!("called `function`");
}

mod deeply{
    pub mod nested{
        pub fn function(){
            println!("called `deeply::nested::function`");
        }
    }
}

fn main() {
    function();
    other_function();

    {
        println!("entering blocks");
        use crate::deeply::nested::function;
        function();
    }

    function();
}
