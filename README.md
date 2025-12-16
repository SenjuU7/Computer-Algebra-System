### 🧮 Symbolic Algebra Engine in Rust

This project is a **minimal symbolic algebra system (CAS prototype)** written in Rust.
It represents mathematical expressions as **expression trees (AST)** and evaluates them using **complex number arithmetic**.

### ✨ Features

* Symbolic expression representation (AST)
* Support for variables and environments
* N-ary addition and multiplication
* Unary operations: negation, sin, cos
* Power expressions
* Complex number evaluation using `num-complex`
* Separation of expression construction and evaluation

### 📌 Example

```text
(a + bi)(c + di) = (ac − bd) + (ad + bc)i
```

This identity is constructed symbolically and evaluated numerically at runtime.

### 🛠 Technologies

* Rust
* `num-complex`
* Expression Trees
* Symbolic Computation

### 🎯 Purpose

This project is intended as:

* A learning project for **symbolic computation**
* A foundation for building a **Computer Algebra System (CAS)**
* An exploration of **AST-based interpreters** and algebraic structures

---

## 🔹 Tags / Topics (GitHub Topics)

```
rust
symbolic-math
computer-algebra
expression-tree
ast
complex-numbers
compiler-design
```
