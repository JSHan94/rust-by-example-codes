enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x : i64, y : i64}
}

fn inspect(event :WebEvent){
    match event{
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(x) => println!("key pressed {}", x),
        WebEvent::Paste(x) => println!("paste {}", x),
        WebEvent::Click{x,y} => println!("click {} {}", x,y),

    }
}

enum VeryVerboseEnum {
    Add,
    Substract
}

type VVE = VeryVerboseEnum;

impl VVE {
    fn run(&self, x:i32, y:i32)-> i32{
        match self{
            Self::Add => x+y,
            Self::Substract => x-y
        }
    }
}

fn main() {
    let pageload : WebEvent = WebEvent::PageLoad;
    let pageunload : WebEvent = WebEvent::PageUnload;
    let click : WebEvent = WebEvent::Click{x:1, y:2};
    let paste : WebEvent = WebEvent::Paste(String::from("test"));
    let keypress : WebEvent = WebEvent::KeyPress('c');

    inspect(pageload);
    inspect(pageunload);
    inspect(click);
    inspect(paste);
    inspect(keypress);

    let vve = VVE::Add;
    println!("vve : {}",vve.run(3,2));
}
