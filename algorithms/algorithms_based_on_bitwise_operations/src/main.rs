// Source: https://www.hackerearth.com/practice/basic-programming/bit-manipulation/basics-of-bit-manipulation/tutorial/
mod utilities; // Declare the module

use utilities::*;

fn main() {
    assert!(is_power_of_2(1));
    assert_eq!(number_of_bits_set(5), 2);
    assert_eq!(number_of_bits_set(0), 0);
    assert_eq!(number_of_bits_set(u32::MAX), 32);
    assert!(ith_bit_set(5, 0));

    let input = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let subsets = possible_subsets(&input);
    assert_eq!(subsets, vec![
        vec![],                 // Empty subset
        vec!["a".to_string()],  // {a}
        vec!["b".to_string()],  // {b}
        vec!["a".to_string(), "b".to_string()], // {a, b}
        vec!["c".to_string()],  // {c}
        vec!["a".to_string(), "c".to_string()], // {a, c}
        vec!["b".to_string(), "c".to_string()], // {b, c}
        vec!["a".to_string(), "b".to_string(), "c".to_string()] // {a, b, c}
    ]);
}

// 4. Generate all the possible subsets of a set ?
// a b c

// 1 1 1

// 101
// 001
// 011
// 010
// 111
// 110
// 100
// 000


// 5. Find the largest power of 2 (most significant bit in binary form), which is less than or equal to the given number N.

// x   = 1001  // 9
// x-1 = 1000  // 8


// x   = 1111  // 15
// x-1 = 1110  // 14


// x   = 1101  // 13
// x-1 = 1100  // 12


// x   = 1011  // 11
// x-1 = 1010  // 10


// x   = 1000  // 8
// x-1 = 0111  // 7. n−1 flips all bits to the right of the single set bit in n and turns the set bit off.
// it then follows that, x & (x-1) = 0000

// 1000 &
// 0111
// --------
// 0000

// Which satisfies that x, is a power of 2 if it has exactly one bit set to 1 in its binary representation.


// Counting set bits
// 1111 &
// 1110
// --------
// 1110 -- 1
// 1101
// 1100 -- 2
// 1011
// 1000 -- 3
// 0111
// 0000 -- 4
