### An Array-Based, Dynamic Memory Allocated Stack Implementation

```
Version: 0.1.0
Date: 2025-01-31
Created by: Jason Armstrong
Last modified: 2025-02-01
```

#### Introduction
This document provides a detailed description of an array-based, dynamically allocated stack implementation in C. The stack is designed as a generic data structure that supports dynamic resizing and can store elements of any data type. This document outlines the system’s architecture, design decisions, and implementation details, serving as a reference for both developers and learners.

#### System Overview
The stack implementation adheres to the following principles:
- A **Last-In-First-Out (LIFO)** data structure.
- A **dynamically allocated array-based** design to allow flexibility in memory usage.
- A set of core **API operations**:
  - **create_stack**: Initializes a new stack.
  - **destroy_stack**: Deallocates memory associated with the stack.
  - **push**: Adds an element to the top of the stack.
  - **pop**: Removes and retrieves the top element.
  - **peek**: Retrieves the top element without removing it.
  - **is_empty**: Checks if the stack is empty.
  - **size**: Returns the number of elements in the stack.

#### Design Considerations

##### **Assumptions and Dependencies**
- The stack will be used in environments where dynamic memory allocation (`malloc`, `realloc`, `free`) is available and functioning correctly.
- The stack assumes that the stored data type has a fixed size per element (i.e., not variable-length structures or self-referential structures requiring deep copies).
- The user is responsible for managing complex types (e.g., ensuring deep copies of dynamically allocated objects if necessary).
- The implementation does not include built-in thread safety; if required, it must be handled externally using synchronization mechanisms such as mutexes.

##### **General Constraints**
- Memory allocation must be efficient to avoid excessive overhead.
- The stack should scale dynamically but avoid excessive memory growth.
- Performance should be optimized for typical push/pop operations (O(1) average case).

##### **Goals and Guidelines**
- Provide an intuitive and reusable stack implementation.
- Support storage of generic data types via `void*`.
- Ensure robust error handling for memory allocation failures and stack underflows.
- Implement a balance between growth efficiency and memory conservation.

##### **Development Methods**
- The implementation follows a modular approach, separating the stack's definition from its functionality.
- Testing is performed using both integer and character-based stack operations.

#### Architectural Strategies
- **Dynamic Memory Allocation**: The stack starts with an initial memory allocation and grows dynamically when full.
- **Resizing Strategy**: If the stack reaches capacity, it expands:
  - Doubles in size if capacity < 1024.
  - Grows by 1.5x otherwise.
  - This approach balances memory usage and reallocation overhead.
- **Memory Management**: `malloc` is used for initial allocation, `realloc` for resizing, and `free` for deallocation.
- **Generic Data Handling**: `void*` allows storing any data type, but the caller is responsible for correct type handling.

#### System Architecture
The stack implementation consists of two main components:
1. **Data Structure (`Stack`)**: Defines the stack attributes, including:
   - `capacity` (total allocated space).
   - `top` (current index for the next insertion).
   - `data_pointer` (pointer to the underlying array buffer).
   - `element_size` (size of each element in bytes).
2. **API Functions**: Provide stack operations:
   - **Memory Management Functions**: `create_stack`, `destroy_stack`.
   - **Stack Operations**: `push`, `pop`, `peek`.
   - **Utility Functions**: `is_empty`, `size`, `ensure_capacity`.

#### Policies and Tactics
- **Error Handling**:
  - If `malloc` or `realloc` fails, an error message is printed, and the operation is safely aborted.
  - Stack underflow (popping from an empty stack) is prevented by checking `top` before popping.
- **Memory Efficiency**:
  - The stack does not shrink automatically to prevent frequent reallocations, but this can be added if needed.
- **Pointer Arithmetic for Efficient Access**:
  - Data access is done via `void*` with explicit byte-wise pointer arithmetic to ensure correct memory offsets.

#### Detailed System Design

##### **Stack Initialization**
```c
Stack* create_stack(uint32_t type_size, uint32_t initial_capacity);
```
- Allocates memory for the stack structure.
- Allocates an array buffer of `initial_capacity` elements.
- Stores the element size for proper memory operations.
- Returns a pointer to the newly created stack or `NULL` if allocation fails.

##### **Push Operation**
```c
void push(Stack* stack, void* element_pointer);
```
- Ensures capacity before adding an element.
- Copies the element to the correct position using `memcpy`.
- Increments `top`.

##### **Pop Operation**
```c
void pop(Stack* stack, void* destination_pointer);
```
- Checks if the stack is empty before proceeding.
- Decrements `top` to access the last element.
- Copies the top element into the provided `destination_pointer`.

##### **Peek Operation**
```c
void peek(Stack* stack, void* destination_pointer);
```
- Similar to `pop`, but does not modify `top`.

##### **Memory Reallocation**
```c
void ensure_capacity(Stack* stack);
```
- If `top == capacity`, the stack’s size is increased using `realloc`.
- If `realloc` fails, an error message is printed, but the stack remains unchanged.

##### **Destruction**
```c
void destroy_stack(Stack* stack);
```
- Frees the allocated memory for the stack and its data buffer.
- Prevents memory leaks by setting freed pointers to `NULL`.

#### Glossary
- **free** - Releases dynamically allocated memory (`free(ptr)`).
- **Heap** - Memory region used for dynamic allocation (`malloc`, `realloc`).
- **malloc** - Allocates memory (`void* malloc(size_t size)`).
- **NULL** - A null pointer, used to indicate an invalid memory reference.
- **Pointer Arithmetic** - Performing calculations on pointers to navigate memory (`ptr + offset`).
- **realloc** - Adjusts previously allocated memory size (`void* realloc(ptr, new_size)`).
- **Stack** - A LIFO data structure that allows adding and removing elements only from the top.
- **void*** - A generic pointer type in C, allowing for storage of any data type.

#### TODO
- Optimize for thread safety
- Add automatic shrinking mechanism.
