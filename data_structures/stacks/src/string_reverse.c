/**
  * Name: string_reverse.c
  * Description: Stack based string reversal
  * Created: 2025-01-31
  * Created by: Jason Armstrong
  *
  * Last modified: 2025-02-01
 */
#include <stdio.h>
#include "../headers/stack.h"

void string_reverse() {
    char ch;
    Stack* stack = create_stack(sizeof(char), 30);  // start with 30 chars before resize. Can move magic number to buffer size?

    // handle failures to stack creation.
    if (!stack) {  // in C, NULL is falsy, and a valid pointer to an object is truthy. Allowing for these idiomatic conditional checks
        printf("ERROR: Could not create stack!\n");
        return;
    }

    printf("Enter text (press Enter to finish): ");
    while ((ch = getchar()) != '\n' && ch != EOF) {
        if (ch != '\r') {  // ignore carriage returns
            push(stack, &ch);
        }
    }

    // check if the user entered any text
    if (stack->top == 0) {
        printf("INFO: No characters entered. Nothing to reverse.\n");
        destroy_stack(stack);
        return;
    }

    // if we have text, proceed to out put its reverse by LIFO
    char reversed_text[stack->top + 1];  // buffer for reversed string. The + 1 is the slot for the null terminator
    int i = 0;
    while (!is_empty(stack)) {
        pop(stack, &reversed_text[i++]);  // use i then post increment it for next iteration
    }
    reversed_text[i] = '\0';  // null-terminate

    printf("Reversed text: %s\n", reversed_text);
    destroy_stack(stack);
}

int main(void) {
    string_reverse();
    return 0;
}
