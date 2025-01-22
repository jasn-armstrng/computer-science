// Source: https://www.hackerearth.com/practice/basic-programming/bit-manipulation/basics-of-bit-manipulation/tutorial/

/// Determines if a given number is a power of 2.
///
/// ### Explanation
/// - A number `x` is a power of 2 if it has exactly one bit set to `1` in its binary representation.
/// - For such a number:
///     - `x - 1` flips all bits to the right of the single set bit in `x` and turns the set bit off.
///     - Performing a bitwise AND (`x & (x - 1)`) results in `0` for powers of 2.
///
/// ### Arguments
/// - `x`: A 32-bit signed integer (`i32`) to check
///
/// ### Returns
/// - `true` if `x` is a power of 2.
/// - `false` otherwise.
///
/// ### Time complexity
/// - O(1)
///
/// ### Examples
/// ```rust
/// assert!(is_power_of_2(1)); // 2^0 = 1
/// assert!(is_power_of_2(2)); // 2^1 = 2
/// assert!(is_power_of_2(4)); // 2^2 = 4
/// assert!(!is_power_of_2(3)); // Not a power of 2
/// assert!(!is_power_of_2(0)); // Not a natural number
/// assert!(!is_power_of_2(-5)); // Not a natural number
/// ```
fn is_power_of_2(x: i32) -> bool {
    if (x > 0) && x & (x-1) == 0 {
        return true;
    }
    false
}

/// Counts the number of 1s in the binary representation of the given natural number.
///
/// ### Explanation
/// - The function uses **Brian Kernighan's method**, which efficiently counts the number of 1s
///   (set bits) in a number's binary representation.
/// - For each iteration, the expression `x = x & (x - 1)` clears the rightmost set bit in `x`.
/// - The process continues until all bits are cleared (i.e., `x` becomes 0).
///
/// ### Edge Cases
/// - If `x = 0`, the binary representation contains no 1s, so the result is 0.
///
/// ### Arguments
/// - `x`: A 32-bit unsigned integer (`u32`) whose set bits are to be counted.
///
/// ### Returns
/// - The number of 1s in the binary representation of `x`.
///
/// ### Time complexity:
/// - O(n), where n is the total number of bits in the integer (32 for u32).
///
/// ### Examples
/// ```rust
/// assert_eq!(number_of_bits_set(5), 2); // 5 in binary is 101
/// assert_eq!(number_of_bits_set(0), 0); // 0 in binary is 0
/// assert_eq!(number_of_bits_set(u32::MAX), 32);  // All bits set for u32::MAX
/// ```
fn number_of_bits_set(mut x: u32) -> u32 {
    let mut count: u32 = 0;
    while x > 0 {
        count += 1;
        x = x & (x-1);
    }
    count
}

fn main() {
    assert!(is_power_of_2(1));
    assert_eq!(number_of_bits_set(5), 2);
    assert_eq!(number_of_bits_set(0), 0);
    assert_eq!(number_of_bits_set(u32::MAX), 32);
}



// 3. Check if the ith bit is set in the binary form of the given number.


// 4. Generate all the possible subsets of a set ?


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
