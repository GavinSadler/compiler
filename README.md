
### Language Design

```
int z = 20;

void main() {
    int x = 5;

    int y = add(x, 10);

    print(y);

    hello_world();
    
    {
        int w = 120;
    }

    print(w); // Error!

    return 0;
}

int add(int a, int b) {
    return a + b;
}

void hello_world() {
    print("Hello world!")
}
```

#### Basic data types

`void, int, float, boolean, string, char`

### Stages

```
Lexer/scanner
    Picks out keywords and scanns things like strings, names, etc.

Parser
    Ensure language structure is valid and generate an Abstract Syntax Tree (AST)
    (Note - In Rust project, lexing/parsing may happen in the same step 🤔)

Declaration Resolution
    Make sure all variables are allocated correctly and scope is respected throughout the program

Typecheck
    Ensure types resolve correctly in the AST

Intermediate Representation (IR)
    Converts our AST into some intermediate type language. Usually assembly-like.

Optimization
    Run through our AST/IR and perform optimizations to the code

Code Generation
    Translate abstract sytax tree into new code format
    We could support:
        Assembly - RISC-V, LLVM, ARM, x86
        C
        Machine code (Pretty much just take our assembly output and translate to machine code. We would be programming an assembler...)
```

### Tools & Resources

Rust
- https://www.rust-lang.org/tools/install

Rustlings
- https://github.com/rust-lang/rustlings/
- Learn how to use Rust with an interactive tutorial

Pest
- https://pest.rs/book

Prof Douglas Thain's CompilerBook
- https://www3.nd.edu/~dthain/compilerbook/compilerbook.pdf
- The book above leaves out a lot of crucial details but is otherwise a great gentle resource as an intro into fullstack compiler construction. It is based on the following:

Compilers: Principles, Techniques, and Tools (The "Dragon Book")
- https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools
- _The OG compiler construction textbook_. Pretty much the above resource but goes into far more detail in actually implementing many of the concepts that are only referenced in Thain's book. Thain's book is derivative of this textbook. A fantastic resource 😊

My WIP B-minor compiler
- https://github.com/GavinSadler/bminor-compiler
- Based on the above 2 resources and Compiler Construction class from university