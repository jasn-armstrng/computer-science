/// Determines if a given number is a power of 2.
///
/// # Explanation
/// - A number `x` is a power of 2 if it has exactly one bit set to `1` in its binary representation.
/// - For such a number:
///     - `x - 1` flips all bits to the right of the single set bit in `x` and turns the set bit off.
///     - Performing a bitwise AND (`x & (x - 1)`) results in `0` for powers of 2.
///
/// # Arguments
/// - `x`: A 32-bit signed integer (`i32`) to check
///
/// # Returns
/// - `true` if `x` is a power of 2.
/// - `false` otherwise.
///
/// # Time complexity
/// - O(1)
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
pub fn is_power_of_2(x: i32) -> bool {
    (x > 0) && x & (x-1) == 0
}

/// Counts the number of 1s in the binary representation of the given natural number.
///
/// # Explanation
/// - The function uses **Brian Kernighan's method**, which efficiently counts the number of 1s
///   (set bits) in a number's binary representation.
/// - For each iteration, the expression `x = x & (x - 1)` clears the rightmost set bit in `x`.
/// - The process continues until all bits are cleared (i.e., `x` becomes 0).
///
/// # Edge Cases
/// - If `x = 0`, the binary representation contains no 1s, so the result is 0.
///
/// # Arguments
/// - `x`: A 32-bit unsigned integer (`u32`) whose set bits are to be counted.
///
/// # Returns
/// - The number of 1s in the binary representation of `x`.
///
/// # Time complexity:
/// - O(n), where n is the total number of bits in the integer (32 for u32).
///
/// # Examples
/// ```rust
/// assert_eq!(number_of_bits_set(5), 2); // 5 in binary is 101
/// assert_eq!(number_of_bits_set(0), 0); // 0 in binary is 0
/// assert_eq!(number_of_bits_set(u32::MAX), 32);  // All bits set for u32::MAX
/// ```
pub fn number_of_bits_set(mut x: u32) -> u32 {
    let mut count: u32 = 0;
    while x > 0 {
        count += 1;
        x = x & (x-1);
    }
    count
}

/// Checks if the `i`th bit is set in the binary representation of a given number.
///
/// # Arguments
/// * `x` - The number whose bits are being checked.
/// * `i` - The 0-based position of the bit to check (from the right, least significant bit).
///
/// # Returns
/// * `true` if the `i`th bit is set (1).
/// * `false` if the `i`th bit is not set (0).
///
/// # Time complexity
/// - O(1)
///
/// # Examples
/// ```rust
/// let number = 10; // Binary: 1010
/// assert!(ith_bit_set(number, 1); // true, The second bit is set.
/// assert!(ith_bit_set(number, 0); // false, The first bit is not set.
/// assert!(ith_bit_set(number, 3); // true, The fourth bit is set.
/// ```
pub fn ith_bit_set(x: u32, i: u32) -> bool {
    let bit_mask: u32 = 1 << i;
    return x & bit_mask > 0;
}

/// Generates all possible subsets of a given set of strings.
///
/// # Arguments
/// * `x` - A slice of strings representing the input set.
///
/// # Returns
/// A `Vec<Vec<String>>` containing all subsets of the input set.
///
/// # Example
/// ```rust
/// let input = vec!["a".to_string(), "b".to_string(), "c".to_string()];
/// let subsets = possible_subsets(&input);
/// assert_eq!(subsets, vec![
///     vec![],                 // Empty subset
///     vec!["a".to_string()],  // {a}
///     vec!["b".to_string()],  // {b}
///     vec!["a".to_string(), "b".to_string()], // {a, b}
///     vec!["c".to_string()],  // {c}
///     vec!["a".to_string(), "c".to_string()], // {a, c}
///     vec!["b".to_string(), "c".to_string()], // {b, c}
///     vec!["a".to_string(), "b".to_string(), "c".to_string()] // {a, b, c}
/// ]);
/// ```
///
// First iteration is 0 & (1 << 0) which evals to 0 & 1, which is 0 three times as per loop, eventually pushing 000 to subset, and then subset to subsets
// Second iteration is 1 & (1 << 0) which evals to 1 & 1, then 01 & 10 which is 0 (false, don't enter loop), then 01 and 100, which is also 0 (false, don't enter loop), so we push 100, or {a} to subset and then subsets
// ...
pub fn possible_subsets(x: &[String]) -> Vec<Vec<String>> {
    let mut subsets = Vec::new(); // To store all subsets
    let n = x.len(); // Number of elements in the input set
    let total_subsets = 2_i32.pow(n as u32); // Total subsets = 2^n

    // Iterate over all possible subset IDs (0 to 2^n - 1)
    for i in 0..total_subsets {
        let mut subset = Vec::new(); // Create a new subset
        for j in 0..n {
            // Check if the jth element is included in the ith subset
            if i & (1 << j) != 0 {
                subset.push(x[j].clone());
            }
        }
        subsets.push(subset);
    }

    subsets
}
