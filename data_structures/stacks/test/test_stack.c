#include <stdio.h>
#include "../headers/stack.h"

void test_stack() {
    Stack* stack = create_stack(sizeof(int), 4);

    int a = 10, b = 20, c = 30;
    push(stack, &a);
    push(stack, &b);
    push(stack, &c);

    int result;
    pop(stack, &result);
    printf("Popped: %d\n", result);  // should print 30

    peek(stack, &result);
    printf("Peek: %d\n", result);  // should print 20

    destroy_stack(stack);
}

int main(void) {
    test_stack();
    return 0;
}
