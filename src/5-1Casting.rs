#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.444_f32;

    println!("{}",decimal);
    // let integer : u8 = decimal; // error

    let integer = decimal as u8;
    let _character = integer as char;
    // let character = decimal as char; // only u8 to char is possible

    println!("1000 as a u16 : {}", 1000 as u16);

    // first 8 LSB (least significant bits) are kept
    // the rest MSB (most significant bits) are truncated
    // 1000 - 256 - 256 - 256 = 232  (2^16 -> 2^8)
    println!("1000 as a u8 : {}", 1000 as u8);
    // - 1 + 256 = 255
    println!(" -1 as a u8 : {}", (-1i8) as u8);
    println!("232 as u8 : {}", 232 as u8);

    // rust casting to bound values (float -> int), Since Rust 1.45
    println!("300.0f32 as u8 : {}", 300.0f32 as u8); 
    println!("-100.0_f32 as u8 : {}", -100.0_f32 as u8);
    println!("nan as u8 : {}", f32::NAN as u8);

    // behavior incures a small runtime cost, can be avoided with unsafe methods
    // but the result might overflow
    unsafe{
        println!("unsafe 300.0 to u8 : {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 : {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
