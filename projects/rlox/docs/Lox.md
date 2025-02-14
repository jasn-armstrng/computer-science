// Notes on the Lox language:
// ==========================
// Dynamically typed
// Automatic memory management
// Primitives (Atoms):
//  - booleans: true, false
//  - number: double-precision floating point,
//  - strings: enclosed in ""
//  - nil: null, or no value
//  - Primitives: #types->enums
// Expressions (Molecules):
// - Arithmetic: binar operations such as a + b, 4 * 5, ...
// - Comparison: ==, !=, >=, <=, >, <
// - Logical operators: !, and, or
// - Precedence and grouping: var average = (min + max) / 2;
// - Expressions produce a value.
// - Expressions: #struct because they are a complex type. e.g,
/*
    struct Unary {
        operator: Operator,
        expr: Expression,
    }

    The above would be the "complex type" representation of for example -5, or !end_of_file().
    The evaluation of the elements of the type in logical order produces a result
*/
// Statements:
//  - Statements produce an effect not a value.
//    - e.g. print "Hello, world!" is a statement (print) that evaluates the expression ("Hello, world!") and displaus the result to the user
//  - Statements are terminated by ";".
//  - An expression terminated by ";" is an expression statement
//  - { } demarcate scopes
//  - Variable declaration: var average = (min + max) / 2;
// Control flow:
//  - if/else
//  - while (loop)
//  - for (loop)
// Functions: fun name(parameter(s)) { body }
// Functions in Lox are first class
/* Closures:

    fun returnFunction() {
    var outside = "outside";

    fun inner() {
        print outside;
    }

    return inner;
    }

    var fn = returnFunction();
    fn();

    Here, inner() accesses a local variable declared outside of its body in the surrounding function.
    Is this kosher? Now that lots of languages have borrowed this feature from Lisp, you probably know
    the answer is yes.

    For that to work, inner() has to “hold on” to references to any surrounding variables that it uses
    so that they stay around even after the outer function has returned. We call functions that do this
    closures.

    As you can imagine, implementing these adds some complexity because we can no longer assume variable
    scope works strictly like a stack where local variables evaporate the moment the function returns.

    #lifetimes #mutability #referencing
*/
/*
Classes:
 - For a dynamically typed language, objects are pretty handy. We need some way of defining
   compound data types to bundle blobs of stuff together.

 - In Lox, you declare a class and its methods like so,

    class Breakfast {
        cook() {
        print "Eggs a-fryin'!";
        }

        serve(who) {
        print "Enjoy your breakfast, " + who + ".";
        }
    }

 - Lox supports class inheritance.
 - Lox is not a pure object-oriented language.
 - See more here on Lox classes at https://craftinginterpreters.com/the-lox-language.html
 - #structs/impl #traits #lifetimes
*/
