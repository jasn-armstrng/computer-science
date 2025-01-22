// Source: https://www.hackerearth.com/practice/basic-programming/bit-manipulation/basics-of-bit-manipulation/tutorial/

// 1. Determine if a given number is a power of 2?
fn is_power_of_2(x: i32) -> bool {
    // Domain: x ∈ N, where N is the set natural numbers, i.e. x >= 1
    // Co-Domain: The result of 2^n where n ∈ {0, N}, i.e. N including 0

    // A number n is a power of 2 if it has exactly one bit set to 1 in its binary representation.
    // For a number n that is a power of 2, n−1 flips all bits to the right of the single set bit in n and turns the set bit off.
    // Then n & (n-1) is a binary number of all zeros
    if (x > 0) && x & (x-1) == 0 {
        return true;
    }

    false
}


fn main() {
    let x: i32 = 0;
    assert!(is_power_of_2(x), "{} is not a power of 2.", x);
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
