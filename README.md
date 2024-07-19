# Rusty Racket

Interpreter for the Racket programming language written in Rust.

## Usage

```
cargo run <filename>
```

## Introduction

Rusty Racket is a Turing complete purely functional language. The name comes from the fact that its interpreter is written in Rust. Rusty Racket is a dialect of the programming language Racket which is a dialect of Scheme which is a dialect of Lisp.

The language includes essential features such as conditionals, user-defined functions, lists, and more. It also comes with a standard library that offers built-in numerical and list functions.

## About the Project

The interpreter is created using a conventional approach. The process is as follows:

1. **Preprocessor**: Resolves module includes.
2. **Tokenizer**: Converts the program string into a list of tokens.
3. **Parser**: Transforms the tokens into an abstract syntax tree.
4. **Interpreter**: Evaluates the abstract syntax tree to produce the final result.

## Examples

All of these examples and more can be found in the `/examples` folder

**Fibonacci**: computes the nth Fibonacci number.

```racket
(define (fibonacci n)
    (cond
        [(= n 0) 0]
        [(= n 1) 1]
        [true (+ (fibonacci (- n 1)) (fibonacci (- n 2)))]))

(define (main) (fibonacci 10)) ; produces 55
```

**List Sum**: computes the sum of the elements of a list.

```racket
(define (list-sum lst)
    (cond
        [(empty? lst) 0]
        [true (+ (car lst) (list-sum (cdr lst)))]))

(define (main) (list-sum (list 1 2 3 4 5))) ; produces 15
```

**List Sort**: sorts a given list using merge sort.

```racket
(include stdlib::list)

(define (__list::sorted-merge-helper lst1 lst2 acc)
    (cond
        [(empty? lst1) (list::append (list::reverse acc) lst2)]
        [(empty? lst2) (list::append (list::reverse acc) lst1)]
        [(< (car lst1) (car lst2)) (__list::sorted-merge-helper (cdr lst1) lst2 (cons (car lst1) acc))]
        [true (__list::sorted-merge-helper lst1 (cdr lst2) (cons (car lst2) acc))]))

(define (__list::sorted-merge lst1 lst2) (__list::sorted-merge-helper lst1 lst2 empty))

(define (list::sort lst)
    (cond
        [(| (empty? lst) (empty? (cdr lst))) lst]
        [(empty? (cdr (cdr lst)))
         (__list::sorted-merge (list (car lst)) (cdr lst))]
        [true
         (__list::sorted-merge (list::sort (list::take lst (/ (list::length lst) 2)))
                               (list::sort (list::drop lst (/ (list::length lst) 2))))]))

(define (main) (list::sort (list 5 1 4 2 3))) ; produces (list 1 2 3 4 5)
```

## Syntax

A program consists of module includes and function definitions. One of the functions must be named main and take no arguments. The result of the program is the output produced by executing the main function.

### Module Includes

There are 2 available standard library modules: `stdlib::num` and `stdlib::list`.

```racket
(include stdlib::num)
(include stdlib::list)
```

### Function Definitions

Since Rusty Racket is purely functional, every function returns exactly one value and produces no side effects.

```racket
(define (<function name> ...arguments) <result>)

(define (add a b) (+ a b))
```

### Numerical and Boolean Operators

Available numerical operators are `+`, `-`, `/`, `*`, and `%` (modulo).

Available boolean operators are `|` (or), `&` (and), `!` (not), `=`, `<`, and `>`.

```racket
(define (main) (+ 1 2))        ; 3
(define (main) (| true false)) ; true
(define (main) (< 5 10))       ; true
```

### Conditional

The conditional expression works the same as in Racket. To create an else branch, the condition `true` can be used.

```racket
(cond
    [condition1 expression1]
    [condition2 expression2]
    [condition3 expression3]
    ...)

(define (max a b)
    (cond
        [(> a b) a] ; if a > b, return a
        [true b]))  ; else return b
```

### Lists

As with other functional programming languages, lists are linked lists with a head and rest. The empty list is represented by `empty`. Lists can be constructed in 2 ways. Using the `list` function or with `cons`.

```racket
(list 1 2 3)
(cons 1 (cons 2 (cons 3 empty)))
```

To get the head of the list, use `car`. To get the rest of the list use `cdr`.

```racket
(car (list 1 2 3)) ; produces 1
(cdr (list 1 2 3)) ; produces (list 2 3)
```

To determine if a list is empty, use the `empty?` function.

```racket
(empty? empty)        ; true
(empty? (list 1 2 3)) ; false
```

To determine if a value is a list, use the `list?` function.

```racket
(list? (list 1 2 3)) ; true
(list? 1)            ; false
```

### Todo

-   [ ] Add support for comments
-   [ ] Add more helpers for nums and bools to alias operations like add, or lteq, gteq
-   [ ] Create a let* statement or with statemement that supports (with ([x 1] [y (+ x 1)] y))

## Future ideas

-   [ ] Support functions as first class citizens
    -   [ ] Implement closures
    -   [ ] Let functions be returned from other functions
    -   [ ] Let functions be a parameter into another function
-   [ ] Support command line arguments into the rusty racket programs
-   [ ] Variadic functions
    -   [ ] If there is exactly a single argument using the keyword argslist
