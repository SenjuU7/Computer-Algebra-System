#🧮 Symbolic Algebra Engine in RustMinimalist, educational prototype of a Symbolic Algebra System (CAS) built entirely in Rust. This project leverages **Expression Trees (AST)** to represent mathematical expressions and evaluates them using **complex number arithmetic**.

##✨ Features* **Symbolic Expression Representation (AST):** Mathematical expressions are structured as abstract syntax trees (expression trees).
* **Complex Number Evaluation:** Utilizes the `num-complex` crate for robust numerical evaluation in the complex plane.
* **Variables and Environments:** Support for defining and substituting symbolic variables at runtime.
* **Core Operations:**
* N-ary Addition (`+`) and Multiplication (`*`)
* Unary Operations: Negation (`-`), Sine (`sin`), Cosine (`cos`)
* Power Expressions (`^`)


* **Separation of Concerns:** Clear distinction between the symbolic expression construction phase and the numerical evaluation phase.

##📌 Example: Complex MultiplicationThe system can handle complex identities, constructing them symbolically and evaluating them numerically at runtime.

###Identity:###Symbolic Construction & Evaluation Process:1. **Construct:** The expression (a + bi)(c + di) is built as an AST.
2. **Define:** An environment maps variables (a, b, c, d) to specific complex values.
3. **Evaluate:** The AST is traversed, and numerical results are calculated using complex arithmetic.

##🛠 Technologies & Concepts| Technology | Description |
| --- | --- |
| **Rust** | The primary language, chosen for its performance, safety, and excellent ecosystem. |
| **`num-complex`** | Crate used for high-precision, built-in support for complex number arithmetic. |
| **Expression Trees** | The fundamental data structure for representing symbolic mathematical structure. |
| **Symbolic Computation** | The core discipline; handling mathematical objects symbolically rather than just numerically. |
| **Compiler Design** | Concepts like AST traversal and interpreter patterns are heavily applied. |

##🎯 PurposeThis project serves as a focused exploration and foundation:

* **A Learning Project:** Deep dive into the mechanics of building a symbolic computation engine from scratch.
* **CAS Foundation:** A clean, minimal base for potentially developing a more extensive Computer Algebra System.
* **AST Exploration:** Hands-on experience with AST-based interpreters and the underlying algebraic structures they manipulate.

##🔹 Ultra-Short DescriptionSymbolic expression tree evaluator with complex numbers, built in Rust.

##🏷️ GitHub Topics```
rust
symbolic-math
computer-algebra
expression-tree
ast
complex-numbers
compiler-design
