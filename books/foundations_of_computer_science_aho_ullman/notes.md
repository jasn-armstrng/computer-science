### Foundations of Computer Science, Aho and Ullman

<br>

#### Chapter 1: Computer Science - The Mechanization of Abstraction

Fundamentally, computer science is a science of _abstraction_ — creating the right model for thinking about a problem and devising the appropriate mechanizable techniques to solve it. Computer scientists must create abstractions of real-world problems that can be understood by computer users and, at the same time, that can be represented and manipulated inside a computer.

**Abstraction** in the sense we use it implies simplification, the replacement of a complex and detailed real-world situation by an understandable model within which we can solve a problem. That is, we “abstract away” the details whose effect on the solution to a problem is minimal or nonexistent, thereby creating a model that lets us deal with the essence of the problem.


**Data structures**
When the data model of the language in which we are writing a program lacks a built-in representation for the data model of the problem at hand, we must represent the needed data model using the abstractions supported by the language. For this purpose, we study data structures, which are methods for representing in the data model of a programming language abstractions that are not an explicit part of that language. Different programming languages may have strikingly different data models. For example, unlike C, the language Lisp supports trees directly, and the language Prolog has logic built into its data model.


**Algorithms**
An algorithm is a precise and unambiguous specification of a sequence of steps that can be carried out mechanically. The notation in which an algorithm is expressed can be any commonly understood language, but in computer science algorithms are most often expressed formally as programs in a programming language, or in an informal style as a sequence of programming language constructs intermingled with English language statements. For example, there are a number of algorithms for sorting the elements of an array, that is, putting the elements in smallest-first order. There are clever searching algorithms such as binary search, which quickly finds a given element in a sorted array by repeatedly dividing in half the portion of the array in which the element could appear.

These, and many other “tricks” for solving common problems, are among the tools the computer scientist uses when designing programs. It is important to know what makes one algorithm better than another. Frequently, the running time, or time taken by an algorithm measured as a function of the size of its input, is one important aspect of the “quality” of the algorithm.

Other aspects of algorithms are also important, particularly their simplicity. Ideally, an algorithm should be easy to understand and easy to turn into a work-ing program. Also, the resulting program should be understandable by a person reading the code that implements the algorithm. Unfortunately, our desires for a fast algorithm and a simple algorithm are often in conflict, and we must choose our algorithm wisely.


**Data Models**
Any mathematical concept can be termed a data model. In computer science, a data model normally has two aspects:
1. The values that objects can assume. For example, many data models contain objects that have integer values. This aspect of the data model is _static_; it tells us what values objects may take. The static part of a programming language’s data model is often called the _type system_.
2. The operations on the data. For example, we normally apply operations such as addition to integers. This aspect of the model is _dynamic_; it tells us the ways in which we can change values and create new values.


**NULL** is a symbolic constant defined in the standard header file stdio.h to be equal to a value that cannot be a pointer to anything.


**Data Model vs Data Structure**
Despite their similar names, a “list” and a “linked list” are very different concepts. A list is a mathematical abstraction, or data model. A linked list is a data structure. In particular, it is the data structure we normally use in C and many similar languages to represent abstract lists in programs.
Each data model is implemented, via data structures and the programs that use them, in some programming language.
