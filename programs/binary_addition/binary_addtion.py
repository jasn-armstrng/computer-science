"""
Binary Increment Algorithm (Simulating Low-Level Carry Mechanism)

1 . Input: A binary number n (e.g., 0b0000) and a carry initialized to 1 (for the increment).

2. Iterate through each bit position from right (least significant bit) to left (most significant bit):
    * Use bitwise operations to check the current bit:
        - (n >> i) & 1 isolates the bit at position i.
    * Determine the new value of the bit after adding the carry:
        - bit + carry.
    * Update the bit value using a combination of AND, OR, and XOR:
        - If the sum is 2 (1 + carry), set the bit to 0 and propagate the carry.
        - If the sum is 1 (0 + carry), set the bit to 1 and stop propagation (carry becomes 0).

3. Handle Overflow:
    If carry remains after the loop, prepend an additional 1 to the binary representation (extend the bit width).

4. Output: The incremented binary number.


Psuedocode

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

def binary_addition(binary_number, carry=1):
    """
    Increment a binary number represented as an actual binary integer.

    :param binary_num: An integer in binary format (e.g., 0b0000).
    :return: The incremented binary number.
    """
    loop = True
    mask = 0b0001
    counter = 0
    while loop:
        current_bit = (binary_number & mask) >> counter
        if current_bit == 0 and carry == 1:
            binary_number = binary_number ^ mask
            carry = 0
            loop = False
        elif current_bit == 1 and carry == 1:
            binary_number = binary_number ^ mask
            # Move mask left by 1 to next bit to compare
            mask = mask << 1
            # Increment counter to track number position to right shift to 1
            counter += 1

    return binary_number


if __name__ == "__main__":
    result = binary_addition(0b1000)
    print(result)
