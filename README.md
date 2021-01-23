# SS (SimpleScript) [![Status](https://img.shields.io/badge/Status-Experimenting%20/%20Pre%20Pre%20Pre%20Technical%20Preview-green.svg)](#project-status)
Just a simple programming language / experiment to build a simpler and less quirky JavaScript.  

> SimpleScript is a Statically Typed, Application programming language inspired by JavaScript/TypeScript and Go, to target multiple execution methods from AOT compilation for binary executables to popular VM platforms (like JVM / BEAM / WASM) to interpretation and JIT compilation techniques.

Here are its core tenets:
1. Simple
2. Simple to Understand --> Intuitive code with no assumptions or quirkiness (WYSIWYG)
3. Simple to Write --> Intuitive semantics without requiring any hackery (WYSIWYG)


## Language design goals
- Read more about the language's spec and design goals in the [specs](./spec.md)
- See the [syntatic grammar definition in BNF](./syntatic%20grammar.bnf)

Generally:
- readability (and in extension, familiarity)
- ease of use (easy and intuitive constructs/syntax)
- no stupid module issues like python
- typed language
- high level language
- memory model
    - either with a gc
    - or something like rust's memory tracking model
- dun be too verbose
- dun be toooo abstract, can support abstract ideas like meta programming, but not too extreme to support less verbosity


## Goals / Milestones
- Stuff to run the language
    - Intepreter implmentation in Rust and JavaScript
    - Rust compiler frontend, hooking up to LLVM
    - Transpilation to JavaScript to run in the web
    - Perhaps a JVM version?
- Others
    - VS code and vim extensions!


## Project Status
Research.. Research.. Research.. Research..  
Currently:
- doing alot of research on programming languages
    - learning more about PLT (Programming Language Theory)
    - studying other languages
- Working on the language spec whilst learning and building upon the research
    - Module system design
    - Researching and experimenting with how to embed Asynchronous programming / Concurrency / Parallel computing into the language semantics itself.
- Learning how to build interpreters / compilers
    - Actively building a interpreter in Rust for a modified lox language, inspired by this [book](https://craftinginterpreters.com/) and [rlox](https://github.com/julioolvr/rlox)
    - The interpreter is not (at least, not yet) for SS, it is for a modified version of of the lox language, which I am building to learn more about building interpreters.


## Author, Credits, License, Contributing
### Author
- [JJ](https://github.com/Jaimeloeuf)

### Credits
This is my first time building my own language 😅 so I had lots of help referencing other similar projects, and here they are:
- [Crafting Intepreters book](https://craftinginterpreters.com/) by [Bob Nystrom](https://github.com/munificent)
- [Java implementation of the lox language](https://github.com/munificent/craftinginterpreters/tree/master/java/com/craftinginterpreters/lox)
- [Rust implementation of the lox language 1](https://github.com/julioolvr/rlox)
- [Rust implementation of the lox language 2](https://github.com/epellis/rlox/)

### License
[MIT](./LICENSE)

### Contributing
Hit me up if you wanna!