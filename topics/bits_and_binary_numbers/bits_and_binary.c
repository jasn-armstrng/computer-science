#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main(void) {
    printf("Size (bytes) of int: %zu\n", sizeof(int32_t));  // 4 bytees
    printf("Size (bits) of int: %zu\n", sizeof(int32_t) * 8);  // 32 bits
    printf("Max value of a 32 bit signed integer in C: %d\n", INT32_MAX);  // 0 (2^31)-1, (e.g.,0 to −2147483648, or 0 to 2147483647)
    printf("Max value of a 32 bit (un)signed integer in C: %u\n", UINT32_MAX); // 0 - (2^32)-1, (e.g., 0 to 4294967295)
    // Notes on the above:
    // 1. In signed 32-bit integer representations, 1 bit, the most significant bit (MSB), which the the left-most bit is reserved for
    //    the values sign i.e. + or -. The remaining 31 bits are used for the magnitude.
    // 2. In unsigned 32-bit integer representations, all 32 bits are used for the magnitude; no bit reservation for signing.
    // 3. In both cases our number representations goes up to 2^(number of bits of magnitude) - 1, because we start counting from 0.
    //    e.g. In a 4 bit scheme there are (2^4), or 16 4-bit combinations, that represents the numbers 0 - 15,
    //    0000 - 0
    //    0001 - 1
    //    0010 - 2
    //    0011 - 3
    //    0100 - 4
    //    0101 - 5
    //    0110 - 6
    //    0111 - 7
    //    1000 - 8
    //    1001 - 9
    //    1010 - 10
    //    1011 - 11
    //    1100 - 12
    //    1101 - 13
    //    1110 - 14
    //    1111 - 15
    // 4. Note 1111, for example is the binary representation of the decimal 15
    // 5. In both signed and unsigned binary representations, the MSB is the leftmost bit.
    // 6. Bits enable us to represent higher-order abstractions by combining them into patterns that map to meaningful data.
    //    For example, while humans intuitively understand decimal numbers (like 15), computers fundamentally operate in binary
    //    (like 1111). Abstractions like decimal numbers, characters, and even programs bridge the gap between human-readable
    //    data and the computer's binary nature.
    // 6.1 Bits (0 and 1) are the fundamental building blocks of data in a computer, meaning
    //      - "All high-level abstractions reduce to combinations of bits"
    return EXIT_SUCCESS;
}
