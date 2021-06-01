# SS (SimpleScript) [![Status](https://img.shields.io/badge/Status-Experimenting%20/%20Pre%20Pre%20Pre%20Technical%20Preview-green.svg)](#project-status)
Just a simple programming language / experiment to build a simpler and less quirky JavaScript.  

> SimpleScript is a Strongly & Statically Typed, Application programming language inspired by JavaScript/TypeScript and Go, to be a simple and clean language with simple semantics to easily target multiple execution methods from AOT compilation for binary executables to popular VM platforms (like JVM / BEAM / WASM) to interpretation and JIT compilation techniques.


## Language design goals
- Read more about the language's spec and design goals in the [specs](./spec.md)
- See the [syntatic grammar definition in BNF](./syntatic%20grammar.bnf)

Core tenets:
1. Simple
2. Simple to Understand --> Intuitive code with no assumptions or quirkiness (WYSIWYG)
3. Simple to Write --> Intuitive semantics without requiring any hackery (WYSIWYG)

Specifically:
- readability (and in extension, familiarity)
- ease of use (easy and intuitive constructs/syntax)
- no stupid module issues like python
- typed language
- high level language
- simple memory model
    - either with a gc
    - or using a borrow tracker like rust
- not too verbose
- not too abstract, e.g. can support abstract ideas like meta programming, but not too extreme as it may reduce readability and in extension, maintainability


## Goals / Milestones
- Reference implementation of the language
    - Intepreter in Rust
    - Rust byte code Virtual Machine
    - A Compiler frontend for LLVM
    - Perhaps a Graal/Wasm/JVM version to target a popular bytecode virtual machine
    <!-- - Transpilation to JavaScript to run in the web -->
- Others
    - Language server
    - VS code and vim extensions


## Project Status
Research.. Research.. and more Research..  
Currently:
- Doing lots of research on programming languages
    - Learning more about PLT (Programming Language Theory)
    - Studying other languages
- Working on the language spec whilst learning and building upon the research
    - Module system design
    - Researching and experimenting with how to embed Asynchronous programming / Concurrency / Parallel computing into the language semantics itself.
- Working on different implementations
    1. [An interpreter](./rust) in Rust for a modified lox language, inspired by this [book](https://craftinginterpreters.com/) and [rlox](https://github.com/julioolvr/rlox)
        - The interpreter is not (at least, not yet) for SS, it is for a modified version of of the lox language, which I am building to learn more about building interpreters.
    2. A [bytecode virtual machine](./rvm) written in rust
        - Just like the interpreter, this is probably not the final version of SS, mainly a modified version of the lox language too, used to experiment with VM design
    3. A [Simply Typed Lambda Calculus](<./Simply Typed Lambda Calculus>) to experiment with lambda calculus and type inference.


## Project layout & Commit style
This mono repo contains the following sub repos and their commit prefixes:
- [ri](./ri)
    - Interpreter written in Rust
    - Commits prefixed with ```ri:``` or ```[rust-i]```
- [rvm](./rvm)
    - Bytecode virtual machine written in Rust
    - Commits prefixed with ```[rvm]``` or ```rvm:```
- [Simply Typed Lambda Calculus](./Simply%20Typed%20Lambda%20Calculus)
    - Simply Typed Lambda Calculus implemented in JavaScript, with a focus on type inference
    - Commits prefixed with ```[stlc]``` or ```stlc:```


## Author, Credits, License, Contributing
### Author
- [JJ](https://github.com/Jaimeloeuf)

### Credits
I had lots of help referencing other similar projects, credits are listed in the README of individual subrepos.

### License
[MIT](./LICENSE)

### Contributing
Hit me up if you wanna!