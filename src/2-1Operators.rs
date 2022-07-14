fn main() {
    println!("{}", 1u32 + 2);
    println!("{}", 1i32 -2); // cannot be u32 due to overflow

    println!("{}", true && false);
    println!("{}", true || false);
    println!("{}", !true);

    println!("{:04b}", 0b0011u32 & 0b0101);
    println!("{:04b}", 0b0011u32 | 0b0101);
    println!("{:04b}", 0b0011u32 ^ 0b0101);
    println!("{}", 1u32 << 5);
    println!("0x{:x}",0x80u32 >>2);

    println!("{}", 1_000_000u32);
}
