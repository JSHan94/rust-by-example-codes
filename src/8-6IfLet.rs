fn main() {
    let optional = Some(8);
    if let Some(i) = optional{
        println!("if let Some(i) = optional");
    } else if false{
        println!("else if false case");
    } else{
        println!("else");
    }

    #[derive(Debug)]
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let bar = Foo::Bar;
    let baz = Foo::Baz;
    let qux = Foo::Qux(33);

    if let Foo::Bar = bar{
        println!("{:?} is Foo::Bar",bar);
    }
    if let Foo::Baz = baz{
        println!("{:?} is Foo::Baz",baz);
    }
    if let Foo::Qux(value @ 1..=100) = qux{
        println!("{:?} is Foo::Qux(value @ 1..=100)",qux);
    }
}
