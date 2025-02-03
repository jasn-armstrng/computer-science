### Understanding void* in C
I want to understand void* in C - motivations, definition, analogs, properties, examples, undefined behaviour, best practices, and anything else one would need to know to wield it responsibly and effectively. I want to take it one small step at a time building up my understanding and skill with it.

---
#### 1. Why does void* exist? (Motivation)
**1.1. What is a Pointer? (Quick Review)**
A pointer in C is a variable that stores the memory address of another variable. Every pointer has:
- A **type**, which determines what kind of data it points to (`int*`, `char*`, `double*`, etc.).
- A **size**, usually 4 or 8 bytes (depending on architecture).
- A **value**, which is the `memory address` it holds.
---
**1.2. The Problem `void*` Solves**
In C, pointers are usually typed. For example:
```C
int x = 42;
int* p = &x;  // p is an int pointer
```
But sometimes, we need **generic pointers** that can point to any data type without knowing what it is beforehand. This is where `void*` comes in.
---
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

**Why Can't We Dereference `void*` Directly?**
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
---
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
---
> C has a heavier cognitive load compared to higher level languages that have the low-level details abstracted away. Remember C assumes the programmer knows what they are doing. C is NOT a toy.
---
**4.2. Writing a Generic Data Structure**
A `void*` is essential when designing a **generic** data structure that can store **any data type**.

Example: Generic Linked List Node
```C
#include <stdio.h>
#include <stdlib.h>  // for malloc use

// Define a generic node
typedef struct Node {
    void* data;   // Generic data pointer. Typically 8 bytes
    struct Node* next;  // Also 8 bytes
} Node;

// Function to create a node
Node* create_node(void* data) {  // note: create node takes a generic function parameter
    Node* new_node = (Node*)malloc(sizeof(Node));  // immediate cast of the void* returned from malloc to a Node pointer (Node*)
                                                   // Here memory is allocated for both pointers - 16 bytes
    if (new_node == NULL) {
        printf("Memory allocation failed\n");
        return NULL;
    }
    new_node->data = data;
    new_node->next = NULL;  // pointer can have the value of a memory address or NULL
    return new_node;

    // Question: why is there no initial capacity for the struct on creation above?
    // Answer: The data function parameter is a pointer, and the Node attribute data is also a pointer.
}

// Function to print an integer stored in the node
void print_int_node(Node* node) {
    if (node != NULL) {
        printf("Data: %d\n", *(int*)node->data);
    }
}

int main() {
    int x = 100;

    // Create a node that stores an int
    Node* head = create_node(&x);

    // Print the data in the node
    print_int_node(head);

    // Free the node
    free(head);

    return 0;
}
```

**4.3. Function Parameters for Generic Data**
Another common use case for `void*` is passing generic data to functions.

Example: qsort() – **The Standard Library Sorting Function**
The C standard library has a **generic** sorting function called `qsort()` that works with any data type.
```C
#include <stdio.h>
#include <stdlib.h>

// Comparison function for integers
int compare_ints(const void* a, const void* b) {
    // If a > b, the result is positive → qsort() swaps them.
    // If a < b, the result is negative → qsort() keeps them as is.
    // If a == b, the result is 0, meaning they are equal.
    return (*(int*)a - *(int*)b);  // cast and dereference to perform integer subtraction
}

int main() {
    int arr[] = {3, 1, 4, 1, 5, 9};
    int n = sizeof(arr) / sizeof(arr[0]);  // Total memory size of array / memory size of an element's type = number of elements in the array, our n

    // Sort the array
    qsort(arr, n, sizeof(int), compare_ints);

    // Print the sorted array
    for (int i = 0; i < n; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    return 0;
}
// More on C's standard qsort() implementation here https://pubs.opengroup.org/onlinepubs/9699919799/
```

**4.4. Interfacing with System Calls**
Many low-level **OS functions** use `void*` to pass data efficiently.

Example: `pthread_create()` for Threading
When creating a thread in C, we often need to pass a function argument of **any type**. `pthread_create()` solves this with `void*`:
```C
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

void* print_message(void* arg) {
    char* message = (char*) arg;
    printf("%s\n", message);
    return NULL;
}

int main() {
    pthread_t thread;
    char* msg = "Hello from thread";

    // Create a new thread
    pthread_create(&thread, NULL, print_message, msg);

    // Wait for the thread to finish
    pthread_join(thread, NULL);

    return 0;
}
```
**Why `void*`?**
* `pthread_create()` must support any type of function argument.
* We cast `void*` to char* inside print_message() to use it correctly.
---
**Summary**
Why is `void*` so useful?
* **Generic Pointers** – Can point to any data type.
* **Memory Allocation** – Used in `malloc()` and similar functions.
* **Generic Data Structures** – Enables flexible linked lists, stacks, trees, etc.
* **Function Parameters** – Helps pass generic data (`qsort()`, `pthread_create()`).
* **Interfacing with System Calls** – Needed for low-level programming.
---
#### 5. Undefined Behavior & Common Mistakes with void* (Avoiding Memory Corruption)
Since `void*` lacks type information, incorrect use can lead to memory corruption, crashes, and undefined behavior. Following are the most common mistakes and how to avoid them.

**5.1. Dereferencing `void*` Directly/Dereferencing Without Casting**
```C
void* ptr;
int x = 42;
ptr = &x;
printf("%d\n", *ptr);  // ❌ ERROR: void* cannot be dereferenced
```
**Why this is a problem?**
* The compiler doesn’t know the size of `void*` or how to interpret its content.
* `void*` is just an address without a type.

**Correct Approach: Cast Before Dereferencing**
```C
printf("%d\n", *(int*)ptr);  // ✅ Correct: Cast void* to int*
```
---
**5.2. Pointer Arithmetic on void***
```C
void* ptr = malloc(4 * sizeof(int));
ptr++;  // ❌ ERROR: Arithmetic on void*
```
**Why This Is a Problem?**
* The compiler doesn’t know the size of `void*`, so ptr++ is invalid.
* Pointer arithmetic requires a type, so the compiler knows how many bytes to move.

**Correct Approach: Cast `void*` to a Typed Pointer**
```C
int* int_ptr = (int*) ptr;  // ✅ Cast to int*
int_ptr++;  // ✅ Moves forward by sizeof(int)
```
---
**5.3. Forgetting to Allocate Memory Before Using void***
```C
void* ptr;
*(int*)ptr = 42;  // ❌ ERROR: ptr is uninitialized (wild pointer)
```
**Why This Is a Problem?**
* ptr is an uninitialized pointer, pointing to an unknown location.
* Writing to it overwrites arbitrary memory, causing undefined behavior.

**Correct Approach: Allocate Memory First**
```C
void* ptr = malloc(sizeof(int));  // ✅ Allocate memory
if (ptr == NULL) { exit(1); }  // ✅ Always check malloc success
*(int*)ptr = 42;  // ✅ Safe assignment
free(ptr);  // ✅ Free memory
```
---
**5.4. Mismatched malloc() and Type/Forgetting sizeof(type) in malloc()**
```C
void* ptr = malloc(4);  // ❌ Assuming some type is always n bytes
```
**Why this is a problem?**
* On some systems, int is 8 bytes, leading to heap corruption or truncation.

**Correct Approach: Use sizeof(type)**
```C
void* ptr = malloc(sizeof(int));  // ✅ Portable allocation
```
---
**5.5 Memory Leaks/Forgetting free()**
```C
void* ptr = malloc(sizeof(int));
ptr = NULL;  // ❌ Memory is leaked, no way to free it
```
**Why This Is a Problem?**
* If we overwrite the pointer without freeing, we lose the reference to allocated memory.

**Correct Approach: Always Free Memory**
```C
void* ptr = malloc(sizeof(int));
free(ptr);  // ✅ Prevents memory leaks
ptr = NULL;  // ✅ NULL out the pointer after freeing
```
---
**5.6. Freeing `void*` Twice (Double Free)**
```C
void* ptr = malloc(sizeof(int));
free(ptr);
free(ptr);  // ❌ ERROR: Double free (undefined behavior)
```
**Why This Is a Problem?**
* Freeing memory twice can lead to heap corruption and segmentation faults.

**Correct Approach: Null the Pointer After Freeing**
```C
void* ptr = malloc(sizeof(int));
free(ptr);
ptr = NULL;  // ✅ Safe: NULL prevents accidental double free
```
---
**5.7. Invalid `void*` to `struct*` Casting**
```C
typedef struct {
    int a;
    double b;
} Data;

void* ptr = malloc(sizeof(int));  // ❌ Incorrect: Allocating int, but using Data*
Data* d = (Data*)ptr;  // ❌ ERROR: Not enough memory for `Data`
```
**Why This Is a Problem?**
* sizeof(int) is smaller than sizeof(Data), leading to heap corruption when accessing b.

**Correct Approach: Allocate Properly**
```C
void* ptr = malloc(sizeof(Data));  // ✅ Allocate memory for `Data`
Data* d = (Data*)ptr;  // ✅ Safe cast
// or Data* d = (Data*)malloc(sizeof(Data));
```
---
**5.8. Using `void*` After free() (Dangling Pointer)**
```C
void* ptr = malloc(10);
free(ptr);
*(int*)ptr = 5;  // ❌ Use-after-free: unpredictable behavior
```
**Safe Approach**
```C
free(ptr);
ptr = NULL;  // ✅ Prevents accidental access
```
---
**5.9. Mixing `void*` and Array Pointers Incorrectly**
A common mistake is forgetting that arrays decay into pointers, leading to unintended behavior.
```C
void* ptr;
int arr[] = {1, 2, 3};
ptr = arr;  // ✅ Valid, but...
printf("%d\n", *(int*)ptr[0]);  // ❌ Wrong: ptr is `void*`, not an array
```
**Correct Approach**
```C
void* ptr;
int arr[] = {1, 2, 3};
ptr = arr;
int* int_ptr = (int*)ptr;
printf("%d\n", int_ptr[0]);  // ✅ Correct indexing
```
---
**Summary: How to Avoid `void*` Mistakes**
| Mistake | Solution |
|---------|----------|
| Dereferencing without casting | Always cast before dereferencing (`*(int*)ptr`) |
| Pointer arithmetic on `void*` | Convert to `char*` or proper type before arithmetic |
| Forgetting to allocate memory | Use `malloc()` before storing data in `void*` |
| Double free | Set pointer to `NULL` after `free()` |
| Using `void*` after `free()` | Reset pointer to `NULL` after freeing |
| Storing stack variables in `void*` | Copy data with `malloc()` and `memcpy()` |
| Memory leaks | Always `free()` allocated memory |
| Casting `void*` incorrectly | Ensure correct type before casting |

---
