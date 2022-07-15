// From, Into와 유사하게 TryFrom, TryInto는 타입간의 컨버팅을 지원하는 trait임
// 하지만 Try에서 알 수 있듯이 conversion이 fallible함

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value : i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }else{
            Err(())
        }
    }
}
fn main() {
    let odd = 5i32;
    let even = 6i32;

    println!("TryFrom odd number : {:?}", EvenNumber::try_from(odd));
    println!("TryFrom even number : {:?}", EvenNumber::try_from(even));
    
    let odd_into : Result<EvenNumber,()> = odd.try_into();
    let even_into : Result<EvenNumber,()> = even.try_into();

    println!("TryInto odd number : {:?}", odd_into);
    println!("TryInto even number : {:?}", even_into);   
}
