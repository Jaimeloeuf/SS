# SS (SimpleScript) [![Status](https://img.shields.io/badge/Status-Experimenting%20/%20Pre%20Pre%20Pre%20Technical%20Preview-green.svg)](#project-status)
Just a simple programming language / experiment to build a simpler and less quirky JavaScript.  

> SimpleScript is a Strongly & Statically typed, Application programming language inspired by JavaScript/TypeScript, SML and Go, to be a simple and clean language that can target multiple execution methods from AOT compilation for binary executables to popular VM platforms (like JVM / BEAM / WASM) to interpretation and JIT compilation techniques.


## Language design goals
- Read the language's specification and design goals in [here](./spec.md)
- See the [Language Design Guidelines here](./Language%20Design%20Guidelines.md)
- See the [syntatic grammar definition in BNF](./syntatic%20grammar.bnf)

### Core tenets
1. Simple
2. Simple to Understand --> Intuitive code with no assumptions or quirkiness (WYSIWYG)
3. Simple to Write --> Intuitive semantics without requiring any hackery (WYSIWYG)

### Specific language goals and features
- Simple to read and understand by using explicit constructs to avoid surprises caused by implicit language constructs
- Simple to write with easy and intuitive constructs and syntax
- Strongly & Statically typed language with type inference and optional type annotations
- High level language so that you don't have to deal with low level constructs of underlying hardware and execution models
- Not too abstract, e.g. can support abstract ideas like meta programming, but not too extreme as it may reduce readability and in extension, maintainability
- No stupid module issues like python


## Project Goals and Milestones
- Reference implementation of the language
    - Intepreter in Rust
    - Byte Code stack based Virtual Machine in Rust
    - A compiler frontend for LLVM
    - Perhaps a Graal/Wasm/JVM version to target a popular bytecode virtual machine
    - Transpilation to JavaScript to run in browsers
- Others
    - Language server
    - VS code and vim extensions


## Project Status
Research.. Research.. and more Research..  
Currently:
- Doing lots of research on programming languages
    - Learning more about PLT (Programming Language Theory)
    - Studying other languages
    - Working on the type system with type inference
- Working on the language spec whilst learning and building upon the research
    - Module system design
    - Researching and experimenting with how to embed Asynchronous programming / Concurrency / Parallel computing into the language semantics itself.
- Working on different implementations
    1. [An interpreter](./rust) in Rust for a modified lox language, inspired by this [book](https://craftinginterpreters.com/) and [rlox](https://github.com/julioolvr/rlox)
        - The interpreter is not (at least, not yet) for SS, it is for a modified version of of the lox language, which I am building to learn more about building interpreters.
        - Currently works with basic static type checking using type inference
    2. A [bytecode virtual machine](./rvm) written in rust
        - Just like the interpreter, this is not the final version of SS, mainly a modified version of the lox language too, used to experiment with VM design
    3. A [Simply Typed Lambda Calculus](<./Simply Typed Lambda Calculus>) to experiment with lambda calculus and type inference.


## Project layout & Commit style
This mono repo contains the following sub repos and their commit prefixes:
- [ri](./ri)
    - Interpreter written in Rust
    - Commits prefixed with ```ri:``` or ```[rust-i]```
- [rvm](./rvm)
    - Bytecode virtual machine written in Rust
    - Commits prefixed with ```rvm:``` or ```[rvm]```
- [vsce](./vsce)
    - Visual Studio Code Extension
    - Will include both the extension and the language server in the future
    - Commits prefixed with ```vsce:```
- [Simply Typed Lambda Calculus](./Simply%20Typed%20Lambda%20Calculus)
    - Simply Typed Lambda Calculus implemented in JavaScript, with a focus on type inference
    - Commits prefixed with ```stlc:``` or ```[stlc]```


## Author, Credits, License, Contributing
### Author
- [JJ](https://github.com/Jaimeloeuf)

### Credits
I had lots of help referencing other similar projects, credits are listed in the README of individual subrepos.

### License
[MIT](./LICENSE)

### Contributing
Hit me up if you wanna!