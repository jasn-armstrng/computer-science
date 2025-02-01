#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdint.h>
// Review of a stack:
// - Abstract data type that can be implemented as an array or linked-list data structure
// - Works on the Last-In-First-Out (LIFO) principle
// - key operations: push, pop, peek, is_empty and sometimes size
// - If array can be fixed-size or dynamic

// Implementation for a dynamically resizing stack for generic types
typedef struct{
    uint32_t capacity;  // current allocated space (number of elements the stack can hold)
    uint32_t top;  // index (or pointer offset) to the next free spot
    void* data_pointer;  // pointer to the array buffer (generic, e.g. void*)
    size_t element_size;  // size of the element (in bytes)
} Stack;

// Stack method declarations
Stack* create_stack(uint32_t type_size, uint32_t initial_capacity);
void destroy_stack(Stack* stack);
void push(Stack* stack, void* element_pointer);
void pop(Stack* stack, void* destination_pointer);
void peek(Stack* stack, void* destination_pointer);  // return top but don't pop
uint8_t is_empty(Stack* stack);  // return a 1 for true, 0 for false
uint32_t size(Stack* stack);  // return the number of elements in the stack
void ensure_capacity(Stack* stack);

// Stack method declarations
Stack* create_stack(uint32_t type_size, uint32_t initial_capacity) {
    Stack* stack = malloc(sizeof(Stack));
    // handle failures to malloc for stack creation
    if (!stack) {
        printf("ERROR: Failed to allocate memory for stack!\n");
        return NULL;
    }
    stack->element_size = type_size;
    stack->capacity = initial_capacity;
    stack->top = 0;  // first index in data_pointer buffer
    stack->data_pointer = malloc(type_size * initial_capacity);
    // handle failures to malloc for stack buffer creation
    if (!stack->data_pointer) {
        free(stack);
        printf("ERROR: Failed to allocate memory for the stack buffer!\n");
        return NULL;
    }
    return stack;
}

void destroy_stack(Stack* stack) {
    if (stack == NULL) {  // then no need to free
        return;
    }
    free(stack->data_pointer);  // free the child first
    free(stack);  // free the parent
}

void push(Stack* stack, void* element_pointer) {
    ensure_capacity(stack);  // ensure space before we push via helper function.
    memcpy((char*)stack->data_pointer + (stack->top * stack->element_size),  // destination address for the new element. (char*)stack->data_pointer ensures pointer arithmetic works correctly - i.e. we copy the element into the correct position.
            element_pointer,  // source
            stack->element_size  // size of new element in bytes
    );
    stack->top++;
}

void ensure_capacity(Stack* stack) {
    if (stack->top == stack->capacity) {  // stack is full
        uint32_t new_capacity = (stack->capacity < 1024) ? stack->capacity * 2 : (stack->capacity * 3) / 2;  // if size < 1024, double stack size, else scale it by 1.5
        void* new_data = realloc(stack->data_pointer, new_capacity * stack->element_size);
        if (new_data == NULL) {
            printf("ERROR: Memory allocation failed!\n");
            return;
        }
        stack->data_pointer = new_data;
        stack->capacity = new_capacity;
    }
}

void pop(Stack* stack, void* destination_pointer) {
    if (stack->top == 0) {  // if stack has at least 1 element top will be 1; the lone element will occupy top-- or position 0.
        printf("ERROR: Stack underflow!\n");
        return;
    }
    stack->top--;  // decrement top so we are at the last element not the next.
    memcpy(destination_pointer,  // caller supplied location to pop element out to.
           (char*)stack->data_pointer + (stack->top * stack->element_size),  // Use pointer arithmetic to get to last elements location
           stack->element_size);
}

void peek(Stack* stack, void* destination_pointer) {  // same as pop but without the decrement to top
    if (stack->top == 0) {  // if stack has at least 1 element top will be 1; the lone element will occupy top-- or position 0.
        printf("INFO: Stack is empty!\n");
        return;
    }
    memcpy(destination_pointer,  // caller supplied location to pop element out to.
        (char*)stack->data_pointer + ((stack->top - 1) * stack->element_size),  // Use pointer arithmetic to get to last elements location without decrementing top. We use a statement (stack->top - 1) that will evaluate to the offset.
        stack->element_size);
}

uint8_t is_empty(Stack* stack) {
    return stack->top == 0;
}

uint32_t size(Stack* stack) {
    return stack->top;
}
