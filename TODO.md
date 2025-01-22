### TODO

#### Bit manipulation

A list of small, useful CLI tools that leverage each bit manipulation operation. These projects will help you understand the concepts while creating something functional:

**Bitwise AND (`&`)**
- [ ] **Permission Checker**
- **Description**: A tool that checks file or user permissions encoded as bit flags (e.g., Unix-style `chmod`).
- **Utility**: Input a permission value (e.g., `755`) and check if specific permissions (read, write, execute) are enabled for user, group, or others.
- **Learning Focus**: Masking and extracting specific bits to determine the state of flags.

---

**Bitwise OR (`|`)**
- [ ] **Flag Setter**
- **Description**: A tool to enable specific flags in a settings configuration.
- **Utility**: Input a current configuration value and a list of flags to enable, then output the updated value.
- **Learning Focus**: Setting specific bits without affecting others.

---

**Bitwise NOT (`~`)**
- [ ] **Subnet Calculator**
- **Description**: Calculate the network and host portions of an IP address based on a subnet mask.
- **Utility**: Given an IP address and subnet mask, calculate and display the network address and broadcast address.
- **Learning Focus**: Use NOT to invert the subnet mask and derive the host portion.

---

**Bitwise XOR (`^`)**
- [ ] **File Integrity Checker**
- **Description**: A tool that computes and verifies XOR-based checksums for file integrity.
- **Utility**: Generate a checksum for a file and verify its integrity against a stored checksum.
- **Learning Focus**: XOR for data integrity, as XOR can detect differences efficiently.

---

**Left Shift (`<<`)**
- [ ] **Bitwise Multiplier**
- **Description**: A tool to multiply numbers by powers of 2 using shifts.
- **Utility**: Input a number and a shift amount, and compute the result, demonstrating how left shifts work in multiplication.
- **Learning Focus**: Efficient multiplication through bit shifts.

---

**Right Shift (`>>`)**
- [ ] **Bit-Level Divider**
- **Description**: A tool to divide numbers by powers of 2 using shifts.
- **Utility**: Input a number and a shift amount, and compute the result, demonstrating how right shifts work in division.
- **Learning Focus**: Integer division and truncation with shifts.

---

**Combined Operations**
- [ ] **Pixel Encoder/Decoder**
- **Description**: Encode and decode RGB values into a single integer and extract individual components.
- **Utility**: Input RGB values to encode them into a 32-bit integer and decode them back into R, G, and B components.
- **Learning Focus**: Use AND, OR, shifts, and masks to manipulate packed data.
<br>

- [ ] **Bitfield Manipulator**
- **Description**: Create a tool to pack, unpack, set, and toggle specific fields in a bitfield.
- **Utility**: Input a bitfield and manipulate specific bits based on user commands (e.g., toggle, clear, set).
- **Learning Focus**: Combine all basic operations to manipulate fields in structured data.

---

**Project Suggestions for Broader Understanding**
- [ ]  **Binary Calculator**:
   - Build a CLI calculator that performs AND, OR, NOT, XOR, left shifts, and right shifts on two binary numbers provided as input.

- [ ] **Network Packet Inspector**:
   - Decode binary-encoded packet headers, extracting fields like protocol type, source address, etc.

- [ ] **Game Simulation**:
   - Create a Conway's Game of Life implementation using bitwise operations to represent and manipulate the grid state.

- [ ] **Compression Tool**:
   - Implement a basic run-length encoding (RLE) or Huffman coding compression tool to manipulate data at the bit level.

---
