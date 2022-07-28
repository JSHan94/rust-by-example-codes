// Phantom type doesn't show up at runtime but checked statically at compile time
// These extra parameters hold no storage values, and have no runtime behavior

use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A,B> (A, PhantomData<B>);

#[derive(PartialEq)] 
struct PhantomStruct<A,B> {first : A, phantom:PhantomData<B>}  
// storage is allocated for generic over `A`, not for `B`


fn main() {
    let _tuple1 : PhantomTuple <char,f32> = PhantomTuple('Q', PhantomData);
    let _tuple2 : PhantomTuple<char,f64> = PhantomTuple('Q', PhantomData);

    let _struct1 : PhantomStruct<char, f32> = PhantomStruct{
        first : 'Q',
        phantom : PhantomData,
    };
    let _struct2 : PhantomStruct<char, f64> = PhantomStruct{
        first : 'Q',
        phantom : PhantomData,
    };
    // println!("_tuple1 == _tuple2 : {}",_tuple1==tuple2); // Error : type mismatch
    
    
    unit_clarification();
}

use std::ops::Add;


// #![attri] : inner attributes - blocks, functions, implementations, modules accepts
// #[attri] : outer attributes - all declarations accepts

#[derive(Debug,Clone,Copy)]
enum Inch{}
#[derive(Debug,Clone,Copy)]
enum Mm{}
#[derive(Debug,Clone,Copy)]
struct Length<Unit>(f64,PhantomData<Unit>);

impl<Unit> Add for Length<Unit>{
    type Output = Length<Unit>;
    fn add(self, rhs: Length<Unit>) -> Length<Unit>{
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn unit_clarification(){ 
    let one_foot : Length<Inch> = Length(12.0, PhantomData);
    let one_meter : Length<Mm> = Length(1000.0, PhantomData);
    
    let two_feet = one_foot + one_foot;
    let two_meter = one_meter + one_meter;

    println!("two feet = {:?}",two_feet.0);
    println!("two meter = {:?}",two_meter.0);
} 
