"""
Binary Increment Algorithm (Simulating Low-Level Carry Mechanism)
=================================================================
1 . Input: A binary number n (e.g., 0b0000) and a carry initialized to 1 (for the increment).

2. Iterate through each bit position from right (least significant bit) to left (most significant bit):
    * Use bitwise operations to check the current bit:
    * Determine the new value of the bit after adding the carry:
        - bit + carry.
    * Update the bit value using a combination of AND, OR, and XOR:
        - If the sum is 2 (1 + carry), set the bit to 0 and propagate the carry.
        - If the sum is 1 (0 + carry), set the bit to 1 and stop propagation (carry becomes 0).

3. Handle Overflow:
    If carry remains after the loop, prepend an additional 1 to the binary representation (extend the bit width).

4. Output: The incremented binary number.


Psuedocode
==========
    INPUT: binary number n, carry = 1
    FOR each bit i in n from least to most significant:
        current_bit = (n >> i) & 1
        if current_bit == 0 and carry == 1:
            SET bit i to 1
            carry = 0
            STOP
        else if current_bit == 1 and carry == 1:
            SET bit i to 0
            carry = 1  (continue to propagate)

    IF carry == 1:
        ADD an additional bit to the left of n (handle overflow)

    OUTPUT: incremented binary number
"""


def binary_increment(binary_number):
    """
    Increment a binary number represented as an actual binary integer.

    :param binary_num: An integer in binary format (e.g., 0b0000).
    :return: The incremented binary number.
    """
    if not isinstance(binary_number, int):
        raise ValueError("Input must be an integer.")

    carry = 1
    current_mask = 0b1  # Bitmask used to iterate over the bits in the binary number
    bit_position = 0
    while True:
        current_bit = (binary_number & current_mask) >> bit_position
        if current_bit == 0 and carry == 1:
            binary_number = binary_number ^ current_mask
            carry = 0  # (carry is resolved,
            break  # stop propagation)
        elif current_bit == 1 and carry == 1:
            binary_number = binary_number ^ current_mask
            # Move current_mask left by 1 to next bit position to compare
            current_mask = current_mask << 1
            # Increment bit_position to track number position to right shift to 1
            bit_position += 1

    # No need to handle overflow here. In python integers are of arbitrary precision,
    # so Python extends the binary representation dynamically inder the hood.
    return binary_number


if __name__ == "__main__":
    assert(binary_increment(-0b10)) == -0b1  # Signed binary
    assert(binary_increment(0b0000)) == 0b0001
    assert(binary_increment(0b0011)) == 0b0100
    assert(binary_increment(0b0101)) == 0b0110
    assert(binary_increment(0b1111)) == 0b10000  # Overflow


"""
Notes:
1. "carry is resolved" when:
   - A 0 bit flips to 1 to "absorb" the carry.
   - At this point, there is no longer a need to propagate the carry to higher bits, and the process stops.
2. One possible improvement would be to add a parameter (n) for number of increments. In the function body
   a loop would perform the increment process n times then return the new number.
"""
