fn main() {
    
    // rust statement는 2가지가 존재함
    
    // variable binding
    let x = 6;
    // expression
    x;
    x+1;
    15;

    // block 또한 expression의 일종으로 변수에 할당하는 것이 가능함
    let y = {
        let tmp1 = 1;
        let tmp2 = 2;
        tmp1 + tmp2
    }
    let z = {
        let tmp1 = 1;
        let tmp2 = 2;
        tmp1 + tmp2; 
    }

    println!("y : {:?}",y);
    println!("z : {:?}",z);
}
