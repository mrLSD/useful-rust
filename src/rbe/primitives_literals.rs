pub fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);
    // type: 1u32, 2u32, 1_u32, 2_u32,

    // Integer subtraction
    println!("1 - 2 = {}", 1_i32 - 2_i32);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    // 1000000u32, 1000_000u32, 1_000_000_u32

    // Hex formatted
    println!("Hex: {}", 0xFC10_ABCD_123D_i64);
    println!("Hex: 0x{:X}", 0xFC10_ABCD_123D_i64);

    // Oct formatted
    println!("Oct as dec: {}", 0o123_456_i64);
    println!("Oct as hex: 0x{:X}", 0o123_456_i64);
    println!("Oct as oct: 0o{:o}", 0o123_456_i64);
    println!("Oct as byte: 0b{:b}", 0o123_456_i64);
}