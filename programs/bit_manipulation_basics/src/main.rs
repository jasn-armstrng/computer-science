// Resource: https://www.hackerearth.com/practice/basic-programming/bit-manipulation/basics-of-bit-manipulation/tutorial/

// Intro: Working on bytes, or data types comprising of bytes like ints, floats, doubles or even data structures which
// stores large amount of bytes is normal for a programmer. In some cases, a programmer needs to go beyond this - that is
// to say that in a deeper level where the importance of bits is realized.

fn main() {
    // Bitwise Operators:
    // There are different bitwise operations used in the bit manipulation. These bit operations operate on the individual
    // bits of the bit patterns. Bit operations are fast and can be used in optimizing time complexity. Some common bit
    // operators are:

    // 1. NOT ( ~ ): Bitwise NOT is an unary operator that flips the bits of the number i.e., if the ith bit is 0, it will
    // change it to 1 and vice versa. Bitwise NOT is nothing but simply the one’s complement of a number.
    // Example:
    println!("Bitwise NOT:");
    let n: u8 = 0b101;  // The binary representation of 5. The full representation for this as a u8 would be 00000101
                        // but the leftmost zeros are truncated when printed. See inline comments at printlns below.
    let bit_mask: u8 = 0b111;  // Introduce a 3-bit mask to limit the operation to 3 bits.
    let not_n = !n & bit_mask;  // Apply NOT, then mask to limit to 3 bits. If no mask was applied then Rust would
                                    // by default returns 11111010 where we see the remainder of the types' leftnmost
                                    // bits flipped.
    println!("    N = {:b}", n);  // binary 101, decimal 5
    println!("   !N = 0{:b}", not_n);  // After applying Bitwise NOT: binary 10, decimal 2
    // In the above !n is evaluated as
    // 101 |
    // 101      Each element of the top row is logically compared with its correspondence in the row below.
    // ----
    // 010      // !(1 & 1) = 0, !(0 & 0) = 1, !(1 & 1) = 0,
    // ----

    println!("");

    // 2. AND ( & ): Bitwise AND is a binary operator that operates on two equal-length bit patterns. If both bits in the
    // compared position of the bit patterns are 1, the bit in the resulting bit pattern is 1, otherwise 0.
    // Example:
    println!("Bitwise AND:");
    let a: u8 = 0b101;  // 5
    let b: u8 = 0b011;  // 3
    println!("    {:b} & 0{:b} = {:b}", a, b, a & b);  // 1
    // In the above a & b is evaluated as
    // 101 &
    // 011      Each element of the top row is logically compared with its correspondence in the row below.
    // ----
    // 001
    // ----

    println!("");

    // 3. OR ( | ): Bitwise OR is also a binary operator that operates on two equal-length bit patterns, similar to bitwise AND.
    // If both bits in the compared position of the bit patterns are 0, the bit in the resulting bit pattern is 0, otherwise 1.
    // Example:
    println!("Bitwise OR:");
    let c: u8 = 0b101;  // 5
    let d: u8 = 0b011;  // 3
    println!("    {:b} | 0{:b} = {:b}", c, d, c | d);  // 7
    // In the above c | d is evaluated as
    // 101 |
    // 011      Each element of the top row is boolean campared with its correspondence in the row below.
    // ----
    // 111
    // ----

    println!("");

    // 4. XOR ( ^ ): Bitwise XOR also takes two equal-length bit patterns. If both bits in the compared position of the
    // bit patterns are 0 or 1, the bit in the resulting bit pattern is 0, otherwise 1.
    // Example:
    println!("Bitwise XOR:");
    let e: u8 = 0b101;  // 5
    let f: u8 = 0b011;  // 3
    println!("    {:b} ^ 0{:b} = {:b}", e, f, e ^ f);  // 6
    // In the above e ^ f is evaluated as
    // 101 ^
    // 011
    // ----
    // 110
    // ----

    println!("");

    // 5. Left Shift ( << ): Left shift operator is a binary operator which shift the some number of bits, in the given
    // bit pattern, to the left and appends 0 at the end. Left shift is equivalent to multiplying the bit pattern with 2^k
    // ( if we are shifting k bits ).
    // Example 1:
    println!("Left Shift:");
    let g: u32 = 0b001;  // 1 * 2^0 = 1
    let h: u32 = 0b010;  // 1 * 2^1 = 2, or k
    match g.checked_shl(h) {
        Some(result) => println!("    {:b} << {:b} = {:b}, or 2^2, or 4", g, h, result),  // 0100 or 4
        None => println!("Overflow detected!")   // 48, or 110000
    }
    // In the above g ^ h is evaluated as
    // g << h = 1 * 2^2 = 4

    // Example 2:
    let i: u32 = 0b00001;  // 1 * 2^1 = 2
    let j: u32 = 0b11000;  // (1 * 2^3) + (1 * 2^4) = 24, or k
    match i.checked_shl(j) {
        Some(result) => println!("    0000{:b} << 0{:b} = {:b}, or 2^24, or {}", i, j, result, result),
        None => println!("Overflow detected!")   // 48, or 110000
    }
    // In the above i ^ j is evaluated as
    // i << j = 1 * 2^k


    // Example 3:
    let k: u32 = 0b000110;  // 2^1 + 2^2 = 6
    let l: u32 = 0b010000;  // 2^4 = 16, or k
    match k.checked_shl(l) {
        Some(result) => println!("    {:b} << {:b} = {:b}, or 6 * 2^16, or {}", k, l, result, result),  // 48, or 110000
        None => println!("Overflow detected!")   // 48, or 110000
    }

    // In the above k ^ l is evaluated as
    // k << l = 6 * 2^16 = 1100000000000000000, or 393216

    // So, x << y = x * 2^y
    // Note: Shifting is usually more efficient than multiplication, as it's a single CPU instruction.
}
