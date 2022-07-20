fn main() {
    // infinite loop
    
    let mut count = 0u32;
    loop {
        count +=1 ;
        if count == 3 {
            continue;
        }

        if count == 4 {
            break;
        }
    }    

    'outer : loop {
        println!("enter outer");
        'inner : loop {
            println!("enter inner");
            break 'outer;
        }
        println!("never reached");
    }
    
    let mut counter = 0; // shadowing
    let result = loop{
        counter +=1 ;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}
