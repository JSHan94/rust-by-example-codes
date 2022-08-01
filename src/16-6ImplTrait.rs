// `impl Trait` can be used
// - as an argument type
// - as a return type

// As an argument type
fn parse_csv_document<R: std::io::BufRead>(src : R) -> std::io::Result<Vec<Vec<String>>>{
    src.lines()
        .map(|line|{
            line.map(|line|{
            line.split(',')
                .map(|entry| String::from(entry.trim()))
                .collect()
            })
        })
        .collect()
}

fn parse_csv_document_refactor(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>>{
    // <R> only used to declare type of 'src', so it can be refactored 
    src.lines()
        .map(|line|{
            line.map(|line|{
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

// As a return type
use std::iter;
use std::vec::IntoIter;
fn combine_vecs_explicit_return_type(
    v : Vec<i32>,
    u : Vec<i32>
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>>{
    v.into_iter().chain(u.into_iter()).cycle()
}
fn combine_vecs(
    v : Vec<i32>,
    u : Vec<i32>
) -> impl Iterator<Item=i32>{
    // exactly same, but simpler
    v.into_iter().chain(u.into_iter()).cycle()
}

// Clousure has its own unnamed concrete type, so you had to allocate it on the heap
// But with `impl Trait` syntax, but we can do it statically like this
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x : i32 | { x + y };
    closure
}

// `impl Trait` can be used to return an iterator that uses `map` and `filter` closure
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > && 0)
        .map(|x| x * 2)
}

fn main(){
    let v1 = vec![1,2,3];
    let v2 = vec![4,5];
    let mut v3 = combine_vecs(v1,v2);
    for i in v3.take(5){
        println!("combine_vecs : {}",i);
    }

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2),3);

    let singles = vec![-3,-2,2,3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4,6]);

}