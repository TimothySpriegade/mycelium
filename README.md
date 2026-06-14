<div align="center">
    <h2>
        <img src=".github/readme/Banner.png" alt="mycelium Banner">
        <br/>
        <strong>the underground network for your code.</strong>
    </h2>
</div>

Mycelium is a small, simple programming language created as a personal project to learn about language design, interpreters, and what goes on under the hood of my favorite programming languages.

The interpreter was written in Go. Using [Writing an Interpreter in Go](https://interpreterbook.com/), but I switched the project to Rust because I wanted to learn more about it. So i rewrote the whole project in Rust.

Furthermore, the language will evolve from an interpreter language to an LLVM-compiled language, as I want to learn more about LLVM and how it works, and also to improve the language's performance.

---

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Building](#building)
  - [Running the REPL](#running-the-repl)
- [Language Syntax](#language-syntax)
  - [Hello World](#hello-world)
  - [Variables & Constants](#variables--constants)
  - [Functions](#functions)
  - [Control Flow](#control-flow)
  - [Data Structures](#data-structures)
  - [Operators](#operators)
- [Project Architecture](#project-architecture)
- [Roadmap](#roadmap)

---

## Overview

Mycelium (`.myc`) is a statically-typed language with a clean, expressive syntax inspired by modern programming languages. It supports:

- **Mutable variables** (`var`) and **immutable constants** (`val`)
- **First-class functions** with explicit type signatures
- **Higher-order functions** — pass functions as arguments
- **Access modifiers** — `prv` (private) for encapsulation
- **Basic data structures** — typed arrays and maps
- **Arithmetic and comparison operators**
- **`if`/`else` control flow**

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- `cargo` (comes bundled with Rust)

### Building

```bash
git clone https://github.com/your-username/mycelium.git
cd mycelium
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

### Running the REPL

Launch the interactive Read-Eval-Print Loop:

```bash
cargo run
```

You will be greeted with:

```
Hello <username>! This is the mycelium programming language!
Have fun trying some commands
>>
```

Type any valid Mycelium expression at the `>> ` prompt. The REPL currently tokenizes your input and prints the resulting token stream — useful for inspecting how the lexer processes code as the evaluator is being built.

---

## Language Syntax

### Hello World

```myc
fnc main(): void {
    std.print("hello world");
}
```

### Variables & Constants

Use `var` for mutable bindings and `val` for immutable constants. Type annotations follow a colon after the name.

```myc
// A mutable integer variable
var score: int = 100;
score = score + 10;

// Immutable constant definitions
val greeting: string = "Welcome to the forest!";
val answer: int = 31 + 11;
```

| Keyword | Mutability | Description              |
|---------|------------|--------------------------|
| `var`   | Mutable    | Can be reassigned        |
| `val`   | Immutable  | Cannot be reassigned     |

### Functions

Functions are declared with `fnc`, followed by the name, typed parameter list, and return type.

```myc
// A simple public function that adds two integers
fnc add(a: int, b: int): int {
    return a + b;
}
```

Use the `prv` modifier to make a function private:

```myc
// A private recursive function to calculate Fibonacci numbers
prv fnc fib(n: int): int {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
```

#### Higher-Order Functions

Functions are first-class values and can be passed as arguments:

```myc
prv fnc apply_and_multiply(f: fnc(int):int, x: int): int {
    return f(x) * x;
}

fnc square(n: int): int {
    return n * n;
}

val result = apply_and_multiply(square, 5); // result = (5*5) * 5 = 125
```

### Control Flow

```myc
if (score > 50) {
    std.print("Passing!");
} else {
    std.print("Try again.");
}
```

### Data Structures

#### Arrays

Typed arrays with a fixed size declaration:

```myc
val flags: array[3][bool] = [true, false, true];
```

#### Maps

Key-value maps with explicit key and value types:

```myc
val permissions: map[string][bool] = {
    "can_read": true,
    "can_write": true,
    "can_execute": false
};
```

### Operators

| Category       | Operators                             |
|----------------|---------------------------------------|
| Arithmetic     | `+`, `-`, `*`, `/`                    |
| Comparison     | `==`, `!=`, `<`, `>`, `<=`, `>=`      |
| Assignment     | `=`                                   |
| Logical/Unary  | `!`                                   |

---

## Project Architecture

The project is structured as a pipeline of compiler stages, each in its own module:

```
src/
├── main.rs              # Entry point — greets user and starts the REPL
├── token/
│   └── token.rs         # Token enum definition and keyword/comparator lookup
├── lexer/
│   ├── mod.rs
│   └── lexer.rs         # Tokenizer — converts raw source text into tokens
├── parser/
│   ├── mod.rs           # Parser struct, parse_program, and helper methods
│   ├── var_statement.rs  # Parses var definition statements
│   ├── val_statement.rs  # Parses val definition statements
│   └── return_statement.rs # Parses return statements
├── ast/
│   ├── mod.rs           # Node trait and Program root node
│   ├── expression/
│   │   └── identifier.rs  # Identifier expression node
│   └── statement/
│       ├── var.rs         # VarDefinitionStatement AST node
│       ├── val.rs         # ValDefinitionStatement AST node
│       └── return_stmt.rs # ReturnStatement AST node
├── types/
│   └── types.rs         # ValidType enum and type resolution helpers
├── repl/
│   └── repl.rs          # Interactive REPL loop
└── reference/           # Example .myc files for syntax reference
    ├── hello_world.myc
    ├── variables_constants.myc
    ├── functions.myc
    └── data_structures.myc
```

### Compilation Pipeline

```
Source Code (.myc)
      │
      ▼
  [ Lexer ]  ──────── Produces a stream of Tokens
      │
      ▼
  [ Parser ]  ─────── Builds an Abstract Syntax Tree (AST)
      │
      ▼
  [ Evaluator ]  ───── (Planned) Tree-walking interpreter
      │
      ▼
  [ LLVM Backend ]  ── (Future) Compile to native machine code
```

---

## Roadmap

- [x] Lexer — tokenizes Mycelium source code
- [x] AST node definitions (`var`, `val`, `return`, identifiers)
- [x] Type system — `ValidType` enum with `int` and `string` support
- [x] Parser — parses `var`, `val`, and `return` statements into an AST
- [x] Interactive REPL (currently prints token stream)
- [ ] Parser — expression parsing (literals, infix, prefix)
- [ ] Tree-walking evaluator / interpreter
- [ ] Standard library (`std.print`, etc.)
- [ ] Type checker
- [ ] LLVM backend for native compilation
- [ ] File execution (`mycelium run <file>.myc`)