// Source: https://www.hackerearth.com/practice/basic-programming/bit-manipulation/basics-of-bit-manipulation/tutorial/

/// Determines if a given number is a power of 2.
///
/// # Domain
/// - The input `x` must be a natural number (`x >= 1`).
///
/// # Co-Domain
/// - The output is a boolean value: `true` or `false`.
///
/// # Explanation
/// - A number `x` is a power of 2 if it has exactly one bit set to `1` in its binary representation.
/// - For such a number:
///     - `x - 1` flips all bits to the right of the single set bit in `x` and turns the set bit off.
///     - Performing a bitwise AND (`x & (x - 1)`) results in `0` for powers of 2.
///
/// # Arguments
/// - `x`: The number to check.
///
/// # Returns
/// - `true` if `x` is a power of 2.
/// - `false` otherwise.
///
/// # Examples
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


fn main() {
    assert!(is_power_of_2(1));
}

// 2. Count the number of ones in the binary representation of the given number.


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
