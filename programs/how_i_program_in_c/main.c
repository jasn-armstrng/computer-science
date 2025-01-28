// Eskil Steenberg's How I program in C: https://www.youtube.com/watch?v=443UNeGrFoM

// I think C is the greatest language, for all the reasons people hate it

// In the beginning all you want is results.
// In the end all you want is control.


// I just want an omelet! vs. What can I make today?
// A good programmer asks What can I make today?


// I spend 20 mins a year looking for memory leaks.
// The rest of the world spends all their time trying to avoid garbage collection.


/* My technology footprint is small */

/*
Features don't impress me.
I use C89, because C99 is too new and... and broken
I don't use all of it. (goto, register, do, continue, auto, volatile)
I have 0 unwrapped dependencies.
The code I write must live for decades.
*/


// Typing is not the problem
// You spend more time reading your own code than the compiler does. Code should be clear and easy to reason about.
// You want the compiler to give you feedback. You want errors; errors are good.
// Everytime the compiler asks you to fix something is one less bug.
// Ambiguity is the enemy.


// Clever is evil. Don't show off.
// We are all dumb, own your dumbness.
// Operator overloading is evil.
// Hidden things for a programmer is bad.


/*
Crashes are good.
Crashes make you fix things
Debuggers are our friends
Tools > language complexity
language should be simple and allow complex structures to be built.
*/

// Naming
// ------------------------------
// Naming: Good code is wide code,
// be descriptive when naming.
// Nameing style guide:
#include <stdint.h>

#define MY_DEFINE
struct MyType {};
int my_function();
int my_variable;  // Variable
//
// Reuse names:
/*
Define words:

array  // array of data
type  // usually and enum
node  // links to other nodes
entity  // Generic networked thing
handle  // Pointer to opaque data structure
func  // Function pointer or function used as a function pointer
internal  // Function internal to a module

i, j , k, count, length, found, next, previous, list, f, vec, ...
*/


// Long functions are good.
// Sequential code is easy to read.
// Write code that does something.
// Alert names: manager, controller, handler. Names of things that don't do anything directly.


// More on naming
// --------------
// object_create()  // like accessing the property of the object, object.create()
// object_destroy()
// object_move()
//
// module_object_create()  // or module.object.create(), accessing property of an object from some module
// module_object_destroy()
// module_object_move()
//
// This makes searching within your codebase easier. Your code has a standard and a clear convention.


/* API Design! Start from the outside and go in i.e. start with the interface */


// Object orientation: 101
// -----------------------
// OO in an OO language:
//      object thing();
//      thing.do_something();
//
// OO in C
//      thing = object_create();  // a function that returns a handle to some data
//      object_do_something(thing);  // a function that does something with that handle.
//
// Small difference.
//
// Void pointers are your friend. [to lookup]
//
// Code and data are separate.
// ---------------------------


// Macros can be useful
// --------------------
// These macros are handy for debugging:
// __FILE__  // String with the file name
// __LINE__  // int with line number


// Let's talk memory
// -----------------
// If something exists, it exists somewhere!
// That is, something can be described by what it is and where it is.
//
// (Programmable) Memory in your computer is a large (virtualized) array of bytes. We do not access actual physical memory.
//
// A pointer is really just an integer number referencing a byte in that memory array.

// Why do pointers have types?
// - Memory does not know about types, but the programmer needs to know so they can access information out of memory.
//   Information such as chars, strings, integers, collections, ...
//
// Pointer examples:
void *p;  // p is an untyped pointer, a void pointer. Useful for information hiding.
short * short_pointer;
int *int_pointer;
// Pointers can be used to reference an address and neighbouring addresses.
// Pointers are the start of an array or bytes in memory.
//
// sizeof in C is not a function. Think of it instead like a cast.
// double a*
// ... malloc(sizeof *a);  [do some testing of this]


// Structs are just a sizeof and a bunch of offset types
// -----------------------------------------------------
typedef struct{
    uint32_t type;
    char name[32];
    float size;
} MyStructType;
// The first member of a struct has the offset zero
//
// Alignment/ordering of our struct's members matter to minimize padding. Padding is unused memory.


// Memory in Computers
// -------------------
// Memory is partitioned into blocks
// Each block has read/write/execute flags
// Block addresses are virtualized
// realloc() is an awesome function.
// Crashes happen when you step outside a block that is not yours.
// [lookup] What is the Mac or Linux equivalent of gflags? gflags is for development environment only.
//
// Retrieval from main memory (RAM) is slow        // Math and binary operations are super fast
// Retrieval from L2 cache is fast
// Retrieval from L1 cache is faster
// Retrieval from register is fastest
//
// Memory coherence is the #1 optimization
//
// Prefer:
typedef struct {
    float width;
    float length;
    // More resource efficient to calculate the area
} MyPlane;
// over,
// typedef struct {
//     float width;
//     float length;
//     float area; // Requires additional memory and introduces an extra step that can be missed
// } MyPlane;
// The above is obviously not going to be the case always, you may need to store/cache areas in applications such as
// computer graphics processing.
//
// As a rule try not to store any data twice.
//
// Striding: Access a subset of data encoded in a type. [lookup and do some examples]


// Architecture
//
// When designing your application
// 1. Identify your primitives
//
//
// Build a mountain
// - Don't be afraid to write code
// - You learn be implementing
// - Do the hard things
// - Everytime you build something you build another part of your mountain
//
// Fixing you code
// If you know your code should be different - fix it now.
// Don't wait.
// It wont be easier in the future.
// No, you won't get more time to do it later
// No, it won't take too much time
// You will earn it back faster than you think.
