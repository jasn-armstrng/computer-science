### Counting schemes

**Schemes:**
- Binary~2~ `{ 0, 1 }`
- Octal~8~ `{ 0, 1, 2, 3, 4, 5, 6, 7 }`
- Decimal~10~ `{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 }`
- Hex~16~ `{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, a, b, c, d , e, f }`

**Example 1:**
- Binary: 101, 5 in decimal
- Octal: 17, 15 in decimal
- Decimal: 5
- Hex: 0xf, 15 in decimal

**Example 2:**
The decimal number 10's representation in the bases:
 - Binary: 1010
 - Octal: 12, or 1 *  8^1  2
 - Hex: a

**Example 3:**
The decimal number 65's representation in the bases:

    Binary:   2^9 |  2^8 |  2^7 |  2^6 |  2^5 |  2^4 |  2^3 |  2^2 |  2^1 |  2 ^ 0 (twos)
                  |      |      |    1 |    0 |    0 |    0 |    0 |    0 |      1         (1 * 2^6) + (1 * 2^0)

    Octal  :  8^9 |  8^8 |  8^7 |  8^6 |  8^5 |  8^4 |  8^3 |  8^2 |  8^1 |  8 ^ 0 (eigths)
                  |      |      |      |      |      |      |    1 |    0 |      1         (1 * 8^2) + (1 * 2^0)

    Decimal: 10^9 | 10^8 | 10^7 | 10^6 | 10^5 | 10^4 | 10^3 | 10^2 | 10^1 | 10 ^ 0 (ones)
                  |      |      |      |      |      |      |      |    6 |      5         (6 * 10^1) + (5 * 10^0)

    Hex:     16^9 | 16^8 | 16^7 | 16^6 | 16^5 | 16^4 | 16^3 | 16^2 | 16^1 | 16 ^ 0 (sixteenths)
                  |      |      |      |      |      |      |      |    4 |      1         (4 * 16^1) + (1 * 16^0)
