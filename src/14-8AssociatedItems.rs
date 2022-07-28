// extension to trait generic, and allow traits to internally define new items (associated type)
struct Container(i32,i32);
trait Contains<A,B> {
    fn contains(&self, _: &A, _: &B) -> bool;   // Explicitly requires `A` or `B`
    fn first(&self) -> i32;                     // Not explicitly requres `A` or `B`
    fn last(&self) -> i32;                      // Not explicitly requres `A` or `B`
}
impl Contains<i32,i32> for Container{
    fn contains(&self, number_1 : &i32, number_2 : &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self.0}
    fn last(&self) -> i32 { self.1}
}
fn difference<A,B,C>(container: &C) -> i32 where
    C : Contains<A,B>
{
        container.last() - container.first()
}

// use `Asscoiated types` to improve readability
struct AssociatedContainer(i32,i32);
trait AssociatedContains{
    type A;
    type B;
    fn contains(&self, number_1 : &Self::A, number_2 : &Self::B) -> bool;
    fn first(&self)->i32;
    fn last(&self)->i32;
}
impl AssociatedContains for AssociatedContainer{
    type A = i32;
    type B = i32;
    fn contains(&self, number_1 : &Self::A, number_2 : &Self::B) -> bool{
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {self.0}
    fn last(&self) -> i32 {self.1}
}
fn associated_difference<C: AssociatedContains>(container : &C) -> i32{
    container.last() - container.first()
}


fn main() {
    let number_1 = 3;
    let number_2 = 10;
    
    println!("----------------------Non Associated Types----------------------------- ");
    let container = Container(number_1,number_2);
    println!("Does container contains? : {}", container.contains(&number_1,&number_2));
    println!("First : {}", container.first());
    println!("Last : {}", container.last());
    println!("Difference : {}", difference(&container));

    println!("----------------------Associated Types----------------------------- ");
    let associated_container = AssociatedContainer(number_1,number_2);
    println!("Does container contains? : {}", associated_container.contains(&number_1,&number_2));
    println!("First : {}", associated_container.first());
    println!("Last : {}", associated_container.last());
    println!("Difference : {}", associated_difference(&associated_container));

}
