### **Stacks**

A `stack` is an example of an _abstract data type_ that can be implemented using arrays or linked lists in memory. A stack has the following behaviors/functions:

- **push(value) (or add(value))** Places an entity onto the top of the stack. (Typically \( O(1) \))

- **pop() (or remove())** Removes an entity from the top of the stack, and returns it. (Typically \( O(1) \))

- **top() (or peek())** Looks at the entity at the top of the stack without removing it. (Typically \( O(1) \))

- **isEmpty()** A boolean value: **true** if the stack is empty, **false** if the stack has at least one element.

> **Note:** Attempting to call `pop()` or `top()` on an empty stack causes a **stack underflow** error. If the stack is implemented with a fixed-size array and is full, pushing onto it may cause **stack overflow**.

```
+------------------------------------------------------------------------------+
| An Abstract Data Type is a mathematical model for a certain class of data    |
| structures that defines the data type purely in terms of its behavior rather |
| than its implementation.                                                     |
| An ADT specifies:                                                            |
| 1. The set of values the type can hold.                                      |
| 2. The set of operations that can be performed on the values.                |
| 3. The rules (axioms/invariants) governing those operations.                 |
| However, an ADT does NOT specify how the data is stored or how the operations|
| are implemented—only how they behave and interact.                           |
+------------------------------------------------------------------------------+
```

**Why “stack”?** Because we model it after a physical stack of objects, where only the item on the top is directly accessible:

```
push -> [  4  ] -> pop
        [  2  ]
        [  5  ]
        [  7  ]
        [  3  ]
```
**Fig. 1.1.** A Stack

Since the last item pushed is the first item removed, a stack is known as a **Last-In-First-Out (LIFO)** structure.

Stacks are frequently used despite (and sometimes because of) their limitations. Many computer architectures implement a hardware stack at the core of their instruction sets—both **push** and **pop** are common assembly instructions.

---

#### **Stacks in Function Calls**

When a function is called, a new _stack frame_ is pushed onto the **call stack** to store the function’s local variables, return address, and other context. When the function returns, its stack frame is popped, restoring the previous function’s state.

Consider the following C-style example:

```c
function_1() {
    function_2();
    return;
}

function_2() {
    function_2();
    return;
}

int main() {
    function_1();
    return 0;
}
```
Here, **main** calls **function_1**, which calls **function_2**, which in turn calls **function_3**. Functions return in the reverse order they were called, illustrating the LIFO pattern.

---

#### **Limitations of Stacks**

- **No random access**
  You only have access to the top element.
- **No direct searching**
  You cannot walk through the stack to find an element without popping those above it.

#### **Advantages of Stacks**

- **Constant time push/pop**
  The push and pop operations are typically \( O(1) \).
- **Wide range of applications**
  Many real-world problems can be effectively solved or modeled by a LIFO mechanism (e.g., backtracking, expression evaluation, undo/redo functionality).

---

####  **Examples of stack usage**
- Checking balanced parentheses in an expression
- Evaluating postfix (Reverse Polish) notation
- Backtracking in algorithms (e.g., Depth-First Search)


**Source:** https://web.stanford.edu/class/archive/cs/cs106b/cs106b.1186/lectures/05-Stacks_Queues/5-Stacks_Queues.pdf
