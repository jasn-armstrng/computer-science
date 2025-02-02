### Understanding void* in C
I want to understand void* in C - motivations, definition, analogs, properties, examples, undefined behaviour, best practices, and anything else one would need to know to wield it responsibly and effectively. I want to take it one small step at a time building up my understanding and skill with it.

#### 1. Why does void* exist? (Motivation)

**1.1. What is a Pointer? (Quick Review)**
A pointer in C is a variable that stores the memory address of another variable. Every pointer has:
- A **type**, which determines what kind of data it points to (`int*`, `char*`, `double*`, etc.).
- A **size**, usually 4 or 8 bytes (depending on architecture).
- A **value**, which is the `memory address` it holds.

**1.2. The Problem void* Solves**
In C, pointers are usually typed. For example:
```C
int x = 42;
int* p = &x;  // p is an int pointer
```
But sometimes, we need **generic pointers** that can point to any data type without knowing what it is beforehand. This is where `void*` comes in.

**1.3. Common Scenarios Where void* is Useful**
1. Generic Data Structures
 * Implementing linked lists, stacks, queues, or trees that store elements of any type.
2. Dynamic Memory Allocation (malloc, calloc, realloc)
 * The standard malloc() function returns a void* because it doesn’t know what type you need.
3. Polymorphic Function Parameters
 * Functions that accept various data types (e.g., sorting functions).
4. Interfacing with Low-Level System Calls
 * Many OS or hardware interfaces use void* to handle arbitrary data.

---

#### 2. What is void*? (Definition & Syntax)
void* is a pointer type that can store the address of any data type. However:
* **It does not** have a specific type associated with it.
* **It cannot be dereferenced** directly (because the compiler doesn't know what it points to).
* **It requires explicit casting** to be useful.

Basic syntax:
```C
void* ptr;   // A void pointer, can store the address of any type
int x = 42;
ptr = &x;    // Storing address of an int in a void pointer
```

However, to use `ptr`, we must **cast it back** to the appropriate type:
```C
int* int_ptr = (int*) ptr; // cast operation on rhs, then assign value (memory address) to lhs
printf("%d\n", *int_ptr); // 42
```

**Why Can't We Dereference void* Directly?**
```C
printf("%d\n", *ptr); // Error! The compiler doesn’t know the type
```
The compiler will complain because it does not know how many bytes to read.

---

#### 3: Analogies for Understanding void*
* **"A Locker Without a Label":**
Imagine a storage locker that can hold anything, but you must know what’s inside before taking something out.
* **"Raw Memory Address Without Type Information":**
Think of void* as a raw memory address. It can store any address but needs explicit instructions on how to interpret the data.


#### 4: Real-World Examples of void*

**4.1. Dynamic Memory Allocation (malloc, calloc, realloc)**
The most common use of `void*` is in memory management functions like `malloc()`. These functions return a `void*` because **they allocate memory without knowing what type the caller needs.**

Example: Allocating Memory for an `int`
```C
#include <stdio.h>
#include <stdlib.h>  // Required for malloc

int main() {
    // Allocate memory for a single integer
    // The void* ptr has a size (bytes) and a memory address (the first block of the underlying array of bytes in virtual memory for the size).
    void* ptr = malloc(sizeof(int));  // malloc returns a void* or NULL
    // [0x...][][][]  assuming an int is 4 bytes on an example system our void*'s value
    // is the address at the first byte.

    // Check if allocation succeeded
    if (ptr == NULL) {  // alternatively if (!ptr) as valid pointer is truthy
        printf("Memory allocation failed\n");
        return 1;
    }

    // Cast the void pointer to an int pointer
    int* int_ptr = (int*) ptr;
    *int_ptr = 42;

    printf("Value: %d\n", *int_ptr); // Output: 42

    // Free allocated memory
    free(ptr);

    return 0;
}
```
**Key Points**
* `malloc(sizeof(int))` allocates 4 bytes of memory (or 8 bytes on some systems).
* Since `malloc()` returns a `void*`, we **must** cast it to `int*` before use.
* We **must** `free()` the allocated memory to prevent leaks.
