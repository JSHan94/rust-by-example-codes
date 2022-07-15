// type statement를 이용하여 기존에 존재하는 타입에 이름을 지정할 수 있음
// type은 반드시 UpperCamelCase 형태의 이름이어야함 (그렇지 않을 경우 warning)
// primitive 타입의 경우엔 해당룰에서 제외 

type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds : NanoSecond = 5 as u64_t;
    let inches : Inch = 7 as u64_t;

    println!("{} ns, {} inch", nanoseconds, inches);
}
