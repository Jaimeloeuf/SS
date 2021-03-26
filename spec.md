# SS (SimpleScript)
- Simple
- Simple to Understand --> Intuitive code with no assumptions or quirkiness (WYSIWYG)
- Simple to Write --> Intuitive semantics without requiring any hackery (WYSIWYG)

Like all other languages, here is a concise introduction to SS.
> SimpleScript is a Statically Typed, Application programming language inspired by JavaScript/TypeScript and Go, to target multiple execution methods from AOT compilation for binary executables to popular VM platforms (like JVM / BEAM / WASM) to interpretation and JIT compilation techniques.


## Features and Goals
Goal is to build a idiot/beginner proof language, because I keep saying "I'm so dumb" when I waste days trying to figure stuff out because the languages that I work with is not intuitive / not what you see is what you get. So I want to build a new language that is Simple, Flexible and Idiot proof while staying practical.  

The main priority of this language is to optimize for reading code more than writing code.  
Code is assumed to be read more often than written, thus it should be easy to read, understand and reason about even if it means sacrificing some implicit assumptions made in other programming languages.  
In short this means that the language semantic will generally favour explicit definitions rather then implicit ones to make the code more readable and having a language syntax that makes its semantic clear and non-ambiguous.  

Inspirations include Javascript / Typescript / Go / Rust.

- Focus on explicit representation of ideas via code. Instead of like JS where there are alot of assumptions/quirks/implicit behaviours/magic
- Application programming language, where memory management is abstracted away
    - See [Memory section](#memory)
- Hardware independent
    If I write the code on a 64bit x86 platform, it should perform the SAME exact way on a 32bit RISC platform.
- Implementation independent
    - The language should be one that can be implemented in any programming language and can be runned in any format from AOT binary executables to direct interpretation
    - And obviously they should work the exact same way, just one faster and native to the platform
    - @todo If compiled, any constants defined at compile time, will be preprocessed to replace the values directly in the code? or will LLVM take care of this?
    - @todo We also need to take care of cross compilation techniques.
- Package management like npm, allow user to pass in a hash for a module, so that when downloading, the tool should verify it...
- Multi Paradigm
    - Procedural
    - Functional
    - Reactive (Some form of this, most likely by introducing a event loop implementation in the standard library)
    - Metaprogramming
- Statically typed
    - Need to know the type at Compile time if compiled
    - The code is always "compiled" first into an IR that can be parsed later on
        - So even when interpreted, Types can be enforced, kinda like TS
- Immutable
    - no data can be changed once created.
- Expressive and extensible using metaprogramming concepts


## Implementation details
One of the aim of SimpleScript is that, the spec should be simple and flexible enough to be implemented in all sorts of ways deemed useful. Thus the goal is to build a few reference implementations for the top few popular stacks right now.  
Thus some of the WIP reference implementations are (sorted by order of development):
1. Interpreted just like JavaScript using a custom intepreter (Tree walk interpreter for simplicity)
    - Intepreter built using Rust/Other langs
    - Might support JIT integrations, but... its damn difficult so tbd
2. Compile to native binaries using LLVM backend and a custom frontend
3. Compile to WASM as this will be used more and more compared to other VMs like JVM / CLR / Erlang BEAM in the future thanks to its sandboxed model and wide language support for Rust/Go/C++/...
    - https://wasmer.io/
    - Instead of providing our own runtime, rely on the WASM runtime...
4. Compile to bytecode for VMs like the JVM / CLR / Erlang BEAM to support more environments using it
5. Support transpilation options? Like transpile to JS/Rust
    - The purpose of this is to take advantage of their build tools, like rusts memory management system and more.
    - Transpile to JS to make it easier to run and more portable, basically like TypeScript or any dialect of JavaScript, but WASM would be preferred for performance.

Notes:
- For any compilation techniques, the language have to be designed to support seperate compilation like Go, primarily for speed and the ability to link to pre-compiled object files.
    - To achieve this
        - https://stackoverflow.com/questions/2976630/how-does-go-compile-so-quickly
        - https://stackoverflow.com/questions/2976630/how-does-go-compile-so-quickly/49863657#49863657
        - Essentially, simple syntax and good dependency (modules) management

## Keywords and symbols
- All the data types
- Keywords
    - const
    - import
    - export
    - function
    - void? undefined? null?
        - The case for this is that, functions can have no return values, therefore we should perhaps include a null or None to indicate that there is nothing.
        - The other thing to consider is, can constants be set as this undefined? Since they can never be changed afterwards.
        - The difference between undefined and null in JS
            - https://medium.com/@alyz26/undefined-vs-null-c567b539ee71
            - 1 in unintentional missing value, 1 is intentionally missing value set by the programmer
        - see rust on concept of null using Option type https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
- Symbols
    - All the operators [link]()
    - ;
    - ( )
    - { }
    - [ ]
    - Comments
        - //
        - /* */


## Comments
Single line comments
```js
// This is a comment
```

Block comments
```js
/*
    This is a comment
*/
```

## Data types and structures and Value declaration
- No variables
- All values are constants (IMMUTABLE)
    - Note that there is no way of declaring variables, you can only create new constants
    - ? perhaps allow mutable variables, but copying rust, have a unsafe block
        - so variables can only be declared and live in an unsafe block
        - The only reason for this is because imperative paradigm is really difficult without mutable variables
- Strongly typed language
    - generics?
    - do we really want it to be strongly typed? Giving up type inference too?
        - well we should be explicit, but this prevents the language from being expressive.
        - https://softwareengineering.stackexchange.com/questions/209376/is-there-a-correlation-between-the-scale-of-the-project-and-the-strictness-of-th
            - Strict type checking and static type checking aren't the same thing. Python is dynamically typed, but is more strict than C. The advantage of static type checking isn't strictness per se but that types are checked at build time, not run time. I've dealt with many C/C++ issues in my career because of implicit casting.
- Should we enforce explicit typing? Or can we have type inference??
    - esp needed for things like getting a value out from a object
    - but if all the structs have fixed schema, shouldnt we be able to know the type too?
- Types on the left hand side like TS and other languages that support Type inference.
- ? Will there be runtime checks? e.g. accessing values on the array pass its bounds?
    - Will this be a runtime or compile time check? can static analysis work on this?
    - E.g. In Go lang, there are constants, and these do not need to have any type declaration, it is implicit so since my whole language is constants, then... do we really need to have types? Unless we introduce variables, since procedural paradigm is basically impossible without variables...

### Primitives
<!-- consider using this type of int format instead? -->
- void? undefined? null? Optional?
    - void means WILL NOT RETURN / NOT ALLOWED TO RETURN from function
        - means that this function is a pure side effect...
    - Should we support undefined/null? This seems to cause alot of issue in other languages...
        - we need at least 1 to signify not defined right
        - e.g.  req.body.value !== undefined
- unsigned ints
    - u8
    - u16
    - u32
    - u64
    - u128
- ints
    - i8
    - i16
    - i32
    - i64
    - i128
- Aliases
    - ubyte
        - u8
    - byte
        - i8
    - uint
        - u32
    - int
        - i32
    - ulong
        - u128
    - long
        - i128
    - float
        - note that not using double as float is a better word
- String
    - Fixed length char array! Means no need for complex underlying vector stuff for dynamic growable strings
- Bool
    - true
    - false
    - In SS, there are no truthy or falesy literal "values" other then the 'true' and 'false' keywords
        - All expressions can be evaluated to true or false values during runtime, ONLY the Keyword are treated as true or false literal values without the need for evaluation.
        - Meaning that, for example an empty string does not evalutes to true or false
        - Explicit comparision expressions is required to check if the string is empty, e.g. if ("" == "")
        - Refer to [Conditions](#Conditions)
### Special data types
- Object
    - key value maps
    - {}
- struct
    - Objects with fixed schema
    - whereas objects just random KV maps
    - Can there be optional properties?
        - E.g. https://www.typescriptlang.org/docs/handbook/interfaces.html#optional-properties
        - so people can still use struct without falling back to Objects
- Array
    - Can arrays be expanded? Or are they like rust tuples with fixed length?
    - since arrays are hard length
        - can we check n prevent out of bounds error?
        - [1, 2][4] --> invalid
    - []
    - Array of elements with different types...?
        - is this even useful in the first place?
        - What are some scenarios where this would occur and be needed?
- Monads
    - Mainly used for higher level error handling abstractions for cleaner code chaining

### User types
- Allow user to create types? What is the point if there is no support for classes?
- Also what is the point of this? if all the types are just a fixed type of struct?
    - To do schema checking/validation/enforcement?

### Creating constants
```js
const <Type> NAME = value;

// e.g.
const int intValue = intValue;
const long longValue = longValue;
const float floatValue = floatValue;
```
```js
// All values in the object is expected to be constants too!
const Object myObject = {
    int intValue: intValue,
    long longValue: longValue,
    float floatValue: floatValue,
}

// To modify a value in the object, YOU CANT, you have to create a new object
const Object newObject = {
    ...myObject,
    // MUST use the same key, but give it a new/same value to override the default value
    int intValue: newValue,
}

// If you find it very tedious to keep creating new objects when you just change a single value within.
// SUGGESTED: compute the final value first then use it to create the object...
// Else leave that outside the object, until you need it in, then you create a new final anonymous object 
```
```js
// Create the individual values
const int intValue = intValue;
const long longValue = longValue;
const float floatValue = floatValue;

// They share the same type but diff value
const int v = 1, v2 = 2;
// They share the same type and same value
const int v = v2 = 2;

// Using the values with explicit typing
const Object myObject = {
    int intValue: intValue,
    long longValue: longValue,
    float floatValue: floatValue,
}

// Using the values with implicit typing
const Object myObjectWithAutoTypes = {
    intValue,
    longValue,
    floatValue,
}
```
```js
const Array<Number> myArray = [1, 2, 3, 4]
// how to have diff types in the same array? or just dont?
// See how typescript allow multiple types in the same array
```

### Accessing values
- Normal access like by passing in the value
- Since all is constant, not need for thinking about pointers, since everything is fixed pointers that points to the same thing
- objects
    - Accessor syntax
    - myObject.key // to get the value out
- Arrays
    - Accessor syntax
    - myArray[arrayIndex]
- Destructuring syntax

### Memory
- SS will come with a GC as part of its runtime
    - Custom GC as part of the interpreter
    - Or memory management taken care by VM targets like JVM/CLR
    - Or linked to your source code as part of the final compiled executable like Go

## Operators
### logical
- Operators
    - ```not```
    - ```and```
    - ```or```
    - Ternary operator ```(expr) ? (true expr) : (false expr)```

- When executing expressions with logical operations "and" + "or" short circuting will be applied.  
    - "and"
        - If the left hand expression evalutes to ```true```, then the right hand expression WILL be evaluated, and if both is ```true```, then the whole expression will evaluate to ```true```, else ```false```
            - ***\*Note\**** that some scripting languages returns the evaluated value of the last expression if the whole expression evalutes to ```true``` and that allows for shorter code, 'nicer' syntax and certain hackery in some cases, but firstly they make the code less readable, and since SS will [not support truthy and falsey values](#Conditions), doing so causes logical conditionals to fail since an actual Boolean value is expected.
        - If the left hand expression evalutes to ```false```, then the right hand expression WILL NOT be evaluated, and the whole expression will evaluate to false
- Instead of following C style logical operators like && and ||, which are written like that because C also support bitwise operators like & and |, SS will not be supporting bitwise operations now, thus the syntax is kept easier with using keywords instead of special operators.

### Binary / Bitwise
<!-- - ~
    - Negate
- &
    - AND
- |
    - OR
- ^
    - XOR
- << / >>
    - Bitwise shift operators -->
- Will not support binary/bitwise operators to keep the language simpler.  
- Yes some computation will be faster with bitwise operations, but then again, this language is not designed for speed. It is designed for Simplicity / Readability / Maintainability in mind.  
- Also this will allow the language to target more implementations that may not support it like certain VMs.  
- This also removes the need for a special format for different number types like binary and hexadecimal, with formats like 0b1100 and 0xA36F that are common with other languages that implement binary operations. Allowing simpler implementations.

### Math
- +
- -
- *
- /
- %
    - modulo to find remainder
- ^  or  **
    - power operator
    - *Should this be included? How to differentiate "^" from the XOR operator?
    - Perhaps this should be a function under the Math standard library.
    - References
        - <https://stackoverflow.com/questions/4843304/why-is-my-power-operator-not-working>
        - <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Exponentiation>
- Notice that they are no Increment and Decrement operators
    - Say you see some nasty code like ```---a;```
    - Is it valid? That depends on how the scanner splits the lexemes. If the scanner sees it like: ```- --a;```
    - Then it could be parsed. But that would require the scanner to know about the grammatical structure of the surrounding code, which entangles things more than we want. Instead, the maximal munch rule says that it is always scanned like: ```-- -a;```
    - It scans it that way even though doing so leads to a syntax error later in the parser.
    - Similiar to rust and lox
- a += 1     /    a -= 1
    - What about shorthands like these?

### Comparison
- ==
- !=
- \>
- <
- \>=
- <=

Note:
- == and != can be used on all types
- <, >, <=, >= can only be used on numbers
    - Might support different types, e.g. comparisons between floats and ints


## Scope and rules
- lexical/static scoping
- Block scope
    - Used to group a bunch of statement together to share the same local scope.
    ```
    {
        // Code here
    }
    ```
- Functions, conditionals and loop are all block scopes
- variable inheritance in scope
- child scope can always access things in the outer scope
    - however parent scope cannot access things in child scope
    - only upward access
- "this" value?
    - https://www.typescriptlang.org/docs/handbook/functions.html#this
- scoping of arrow functions...


## Conditions
Expressions that evalute to a BOOLEAN ONLY
- unlike JS and other langs, we WILL NOT support truthy and falsey values.
    - and only support true/false
    - meaning I cannot do, value = undefined; if(value)
    - I have to do, value = undefined; if(value !== undefined)
    - To make the logic more EXPLICIT, so yes, more verbose, BUT you will always know if smth fks up
        - if too verbose, we have autocompleting snippets for that... just use smth like,
            - ifu --> map to, --> if($1 === undefined)
            - ifnu --> map to, --> if($1 !== undefined)
    - The reason for this is because too many JS code act weirdly because we forget to check for this truthy and falsey values despite the convienience they give us
- To use conditions, which only support expressions that evaluates to boolean values, comparisons and equality checks can be used to produce these boolean values.


## Control flows
### If/ElseIf/Else
Bracketless one line statements might be supported later on for cleaner code syntax
```js
if (condition) {
    
} else if (condition) {
    
} else {
    
}
```
### Conditional / ternary operator
The Conditional operator gives you the ability to conditionally execute/return expressions, and treat the whole thing as a single expression
```js
const expression = booleanCondition ? trueExpression : falseExpression;
```
### Switch
- ? Should we support rust type of advance pattern matching? Pretty useful but might be quite difficult for beginners
    - This needs enums too right?


## Loops
- no loops
- only functional iterables
- not possible to do loops when all your values are constants, how to do a for loop?
recursion
```js
import iterable from "std:iterable";

const Array myArray = ["value", "value2", "value3"];

iterable(myArray).forEach((value, index) => console.log(`Index: ${index}  Value: ${value}`));
```

## Strings
- Interpolation
    - How do u "print" or stringify it different types like arrays?
    - Must be implemented by the runtime?
- String concat is not supported through + operator overloading
- Memory allocation is entirely up to the implementation heap


## Functions
- Split into pure and impure functions
    - Uses decorators
        - https://www.typescriptlang.org/docs/handbook/decorators.html
        - Are decorators really that useful? In the python and TS sense? Where they are just syntatic sugar for higher order functions and not special language semantics
        - These decorators that I am talking about for pure/impure are language semantics rather than user defined meta progrmaming
- <https://stackoverflow.com/questions/903120/should-i-always-give-a-return-value-to-my-function>
    - Like the answers in this question, perhaps we should seperate pure and impure programming functions using definitions from mathematics. Where functions refer to pure functions following their definitions of taking a input and giving you a ouput, and procedures/subroutines that are simply a group of imperative code.
    - so then, when we define a function, by math definition it is a pure function
    - and if we want a "impure function" so to say, we will define a "subroutine or procedure" instead
    - This makes it harder for user to change functions to subroutines and vice versa. Say they want to log some stuff to console for testing... they wont be able to do so without changing type.
- In function signatures, you must declare the type of each parameter.
- Must also declare return type...
    ```ts
    // Declare in front, because if this is a pure function, we can essentially treat this as a variable definition
    // just like how it is, const TYPE name = value
    // Fns with return first is,  function TYPE name (INPUT)
    function int functionName(int arg) { }
    function functionName(int arg): int { }
    ```
- IIFEs are supported, and used mainly to enclose all the data and logic into its self enclosing scope
- Arrow functions are supported as lambdas / anonymous functions
- Function arguements will be evaluated 1 by 1 in order from right to left before they are passed to the function.
    - The arguments will be evaluated in a strict order to make it easier to read, reason and prove the execution order and control flows. This differs from other languages like Scheme and C where the spec does not specify, giving compilers the freedom to reorder for efficiency.
        - what you see is what you get
- ```return``` keyword
    - The return keyword causes the function to end at any point during execution
    - A function will end when it reaches the end of its function body or when a return keyword is executed
    - A return keyword also allows the user to "return" a value from a function.
        - This can be a literal value, or an expression that evaluates to a value
        - Any function call, is an expression, so it can be expressed as its return value directly.
        - If function g() returns "my string", then all expressions of g() can be replaced with "my string"
            - **Only if the function is pure and referentially transparent**
    - !!!! What happens if return is used outside of a function?
        - this perhaps should be a syntax error
        - for now the resolver will error out on it
        - add a section in spec to address this
- TBD:
    - should functions be hoisted? Or cannot be accessed before definition
    - overloading?
    - Should there be implicit returns for functions? Does that mean we need to support undefined?
    - Named function arguments like in python? Removes the need for overloading and undefined function inputs to pass in a argument later in the sequence
    - C style syntax uses ()
        - but when we do something like nested functions, there end up with too man braces, thus making the elm syntax much much nicer
        - instead of f1(f2(f3(f4(f5(arg)))))
        - elm syntax f1 f2 f3 f4 f5 arg
    - for function calls, should it be pass by value or pass by reference?
        - pass by reference is fine right? Since values cant be modified anyways right.
        - Although if all data is immutable, we can directly pass values by reference instead of duplication the value.
            - @todo Right now, the test intererter evaluates all arguements, then pass this newly created Value objects from evaluating the arguements into the function scope to use.
    - what about variadic functions?
        - like JS provide arguement value?
        - or like JS Rest parameters, using fn(...Args)
        - but if using rest parameters, how do you garuntee the type?
        - When using variadic functions, the code must explicitly identify the function as variadic
    - For non variadic function, should it be a runtime error to pass it argument list of different length?
        - Should arity check be done at runtime or compile/parsing time?
        - If more arguements then parameter list, the arguements after that will be ignored and not be evaluated?
    - default function arguements like JS.
    - If a function that returns something is called and the caller does not use the return value,
        - Should it be considered as an error?
        - The [midori method](http://joeduffyblog.com/2016/02/07/the-error-model/) to make it explicit
            - ignore foo();
            - [Calling ignore method on promises](http://joeduffyblog.com/2015/11/19/asynchronous-everything/)
            - If function have return value, it cannot be ignored, it must be explicitly ignored like in Go lang

### Pure functions
```js
// All functions are assumed to be pure
// If compiler realises that they are not pure, then will throw error
function functionName(args) {
    // Can only access variables in this function scope
    // Cannot access global variables???
    //      What about closures?
    // If I read that variable, then it is no longer pure right...
    // BUT since all the variables are all constants, then it can still remain pure, as the const in here will be replaced by the value itself
    
    // Only allowed to call other pure functions in this scope.
    // BUT advised against doing that. If you want to chain functions tgt, instead use functional composition
}
```
### Impure functions
```js
@impure // Use decorators to explicitly declare these as impure (anything with side effect / none pure input/output type)
function functionName(args) {
    // Can access both pure and impure functions from within this scope
}
// Other type of possible syntaxes
// async function functionName(args) {
// impure function functionName(args) {
// Can async functions be pure??
// This depends on the definition of what our async tag will do to the function.
```
### Gray area / hybrid
Hybrid fns or gray area type def is something like console.log
- Technically, it is a pure function, because for every input, it will always append the same thing to STDOUT
- but it is not a garunteed operation.
- And it is not referentially transparent.
- Thus it is not a pure fuction by strict definition

### Composition
- Use the function from the standard library
- For this, you can pass in mix of pure and impure functions, but no good
- if you pass in pure functions only, compiler will detect it, and can thus optimize / memoize it
- if you put in impure functions, it will still run, but the output of compose, will be marked as impure
```js
import FP from "std:FP";

// Directly chain / compose and run to get results
// This equals to
// f4(f3(f2(f1(inputValueForF1))));
FP.compose(f1, f2, f3, f4, inputValueForF1);

// Create a new chained function to use later
const composition = FP.compose(f1, f2, f3, f4); // This should be the preferred way of using this
const value = composition(inputValueForF1);

// 
FP.composeWithInput(inputValueForF1, f1, f2, f3, f4);

function composeWithInput(input, ...fns){
    return fns.reduce((prev, curr) => curr(prev), input);
}
```

### Memoization
```js
import memoize from "std:memoize";

function pureFunc(arg) {
    // Can only memoize pure functions
}

// passing impure functions into memoize will result in error thrown

const memoizedPureFunc = memoize(pureFunc);

// OR using anonymous functions
const memoizedPureFunc = memoize(function pureFunc(arg) {
    // Can only memoize pure functions
});
```

## Classes? Objects?
- None
- Use the plain JS objects, as key value maps
- for more advance type of objects or stuff
- use a factory function that returns a object with all the values/fields and methods/functions needed

```js
// generic type input
function factoryFunction(<T> constructorArgs) {
    return {
        int intValue: value,
        typeof constructorArgs args: constructorArgs,
        T args2: constructorArgs,
    }
}
```


## Asynchronous programming
### Legend:
- Tasks here generally refers to CPU bound tasks
- Blocking tasks generally refer to Long running CPU bound tasks, or Non CPU (hardware peripheral) bound tasks, e.g counting to 1 million d.p. of Pi, or waiting for Hardrive to respond to a file read request.

### Thoughts, Ideas and Research
- What is the purpose of asynchronous programming?
    1. So that we can do more than 1 thing simultaneously?
        - Both piece of code running at the same time
    2. Or so that we can do more than 1 thing concurrently?
        - Both piece of code running across time T,
        - Both only 1 piece of code running at any given point (Tp) in time T
        - So only 1 executing at once, but overall across T time, more than 1 can execute
            - This is so that when 1 piece of code need to wait for e.g. the network, another CPU bound task can execute.
- Main scenarios for asynchronous programming
    - I/O operations
        - Making a network call
        - Talking to a database
        - File IO
        - Waiting for user input
        - A synchronous program that performs an I/O operation will come to a halt until the operation finishes. A more efficient program would instead perform the operation and continue executing other code while the operation is pending.
        - Say you have a program that reads some user input, makes some computation and then sends the result via email. When sending an email, you have to send some data out to the network and then wait for the receiving server to respond. Time invested by waiting for the server to respond is time wasted that would be of much better use if the program continued computing.
    - Performing multiple operations in parallel
        - When you need to do different operations in parallel, for example, making a database call, web service call and any calculations, then we can use asynchrony.
    - Long-running event-driven requests
        - This is the idea where you have a request that comes in, and the request goes to sleep for some time waiting for some other event to take place when that event takes place, you want the request to continue and then send a response to client.
        - In this case, when request comes in, then thread is assigned to that request and as request goes to sleep, then thread is sent back to threadpool and as the task completes, then it generates the event and picks a thread from thread pool for sending response (the thread sent and picked from thread pool might or might not be the same.
- There are different approaches to implementing each of the 2 purposes above.
    - But the primary purpose that is more common is point (2)


#### How other languages do it
- Python Concurrency
    - https://www.toptal.com/python/beginners-guide-to-concurrency-and-parallelism-in-python
    - Running Python threading script on the same machine for downloading images was 4.7 times faster. While this is much faster, it is worth mentioning that only one thread was executing at a time throughout this process due to the GIL.
    - Therefore, this code is concurrent but not parallel. The reason it is still faster is because this is an IO bound task. The processor hardly breaks a sweat while downloading these images, and the majority of the time is spent waiting for the network.
    - This is why Python multithreading can provide a large speed increase.
    - The processor can switch between the threads whenever one of them is ready to do some work.
    - Using the threading module in Python or any other interpreted language with a GIL can actually result in reduced performance if your code is performing a CPU bound task, such as decompressing gzip files, using the threading module will result in a slower execution time.
    - For CPU bound tasks and truly parallel execution, we can use the multiprocessing module.


> tl;dr  
Single Threaded code just like JavaScript, but users can use the provided event loop library from std lib.  
Supports multi process code by spinning up new native os processes (implemented by os not our runtime).  
For now, no kernel thread support, rather user level thread via thread libraries from std lib.

- Similiar to how JavaScript uses a event based / reactive paradigm to achieve asynchronous programming and concurrency, SimpleScript supports asynchronous programming natively making it as easy as possible for users to do asynchronous programming.
- However the problem with JavaScript's approach is that it limits what the user is able to create because of its single threaded nature. And although JS supports web workers and Node JS supports seperate processes, it is not as ergonomic to the user and can be really confusing even for experienced users, as the Single Threaded nature and event based design can often screw with our existing understanding of how seperate threads and processes work.
    - Not to mention that this makes the language implementation more difficult.
- For newer JS users, async/concurrency concepts are also harder to understand, and more often then not, they have to learn the quirks of the event loop the hard way, which is often painful and time consuming (not simple at all 😫)
- Thus the approach taken by SimpleScript is to combine the best of most worlds as described in the tl;dr by focusing on providing a great bare minimum setup with simple ways to extend it.


## Modules & Libraries
A standard way for splitting up code for sharing and modularity.  
The goal is to provide AN EXTREMELY SIMPLE way of dealing with modules to users. It should be designed for zero cognitive load, as module systems and modularity is a huge source of stress in other languages from C's plaster solution with preprocessor and linking, to python's terrible library module setup, to JS's node_modules dependency graph issue and compatibility issues between ES and Common JS modules.

[A good explaination of how ES modules and Common JS modules work in JavaScript.](https://hacks.mozilla.org/2018/03/es-modules-a-cartoon-deep-dive/) This will be the basis of SS module design for now.

- Modules
    - Every new file is a module. Module --> alias for "file"
    - Support breaking code up into modules.
    - Modules resolution
        - https://www.typescriptlang.org/docs/handbook/module-resolution.html
- Libraries
    - Library, is simply an alias for "file folder"
    - A library can contain both files and more folders
    - A library is a collection of modules and or more libraries, with at least 1 module, to faciliate cross project code sharing
    - The std library is a library containing many sub libraries providing standardised core functionality to projects
- Modules and Libraries can be used to apply namespaces to the code scope
- The name of the imported item MUST be the same as the exported item to make explicit what you are importing
- Modules / Types / Seperate Compilation

### Import
- Import a standard library
    ```js
    import libraryName from "std/libraryName";
    ```
- Import a module from a standard library
    ```js
    import moduleName from "std/libraryName/moduleName";
    ```
- Import a library
    ```js
    import libraryName from "libraryName";
    ```
- Import a module from a library
    ```js
    import moduleName from "libraryName/moduleName";
    ```
- Import a user written module using relative path
    ```js
    import moduleName from "./myModuleName";
    ```
- Import a module into its own namespace instead of the current module's namespace
    ```js
    import "./myModuleName" as moduleName
    ```

### Export
- You must export everything explicitly in order for it to be available to module importer's namespace.
```js
export 
```


## Global includes/preamble
What is always available in the global scope without any import
Default import essentially
- console
- debugger // Keyword, not default import
    - @todo Include a section for this in the spec

    

## Metaprogramming
### Proxies
```js
import proxy from "std:proxy";

// Orig object we want to proxy later
const Object targetObj = {
    int value: value,
}

// 
proxy.new(targetObj)
```

### Macros
- Will consider supporting certain types of macros, but TBD
- Might be an issue considering most macro implementations require preprocessing in some form or another and is extremely complex to take up with subjective level of advantages especially for a language designed to be easy to use and read.


## Language extensions
### FFI (Foreign Function Interface)
- Will have builtin language/std-lib level support for FFI to interact with Rust and C/C++ code in the future
- "linking" mechanisms
    - complie time linking
    - FFI
    - dynamic library linking
    - running the code seperatly and calling the process
- Inspiration from other langs
    - https://www.lua.org/pil/24.html
    - https://wren.io/embedding/


## Printing / Logging mechanisms
Pretty print
Allow us to print diff things like variables to strings to functions...

### Functions
When printing functions, the type of function and the function name will be displayed, where $FUNCTION_NAME is the name of the function.  
- Native functions, defined by the runtime in any other language
    - <function-native: $FUNCTION_NAME>
- Named functions defined in SimpleScript, regardless of whether it is a user defined function or a standard library function.
    - <function-ss: $FUNCTION_NAME>
- Anonymous functions defined in SimpleScript, regardless of whether it is a user defined function or a standard library function.
    - <function-ss: [anonymous]>


## Others
### Why isn't feature "X" included? And what's the workaround?
- enum
    - Might consider supporting but TBD... and for now use the workaround
    - just like in JS, this is not supported, but can be easily worked around using structs/objects
- Augmented assignment (myVar += 1)
    - Well since all values are immutable constants... you cant assign back to yourself anyways...
- Implicit variable declaration
    - SS requires Explicit variable declaration to make it clearer on scoping rules, can only access after defined and in the same scope or deeper scopes.
- One variable per declaration to make things more readable and explicit
    - const x, y = 1, 2; // Not supported by the language
    - const x = 1, y = 2; // Not supported by the language
    - const x = 1; const y = 2; // Use this on 2 lines


## TODO (Things to add to spec)
- Pass by reference or pass by copy?
    - If we continue on, it will be pass by reference by default if all data is immutable
- error and exception handling
    - How do we implement this semantic?
        - try/catch?
        - Monads? Optional Types? Like the go2 draft where there is a "check" keyword to "unwrap the monad" with a nicer syntax
    - For errors, generate error messages like --> error TS2307: Cannot find module 'moduleA'. Then user can use the given link to a website to learn and find out more.
    - other languages
        - https://wren.io/error-handling.html
- A part of the spec should include native code from standard library
    - native code as in, implemented by the runtime, instead of being libraries written in SS itself
        - JSON support
            - Should have JSON DOS protection, by checking length of JSON
                - Either JSON DOS attacks by blocking the CPU thread with long strings
                - Or by parsing into an extremely large object that takes up lots of ram.
            - Or offer libs in std that support Async JSON
                - https://nodejs.org/en/docs/guides/dont-block-the-event-loop/#blocking-the-event-loop-json-dos
        - assertions (Technically can be implemented in SS using a check and throwing on error)
- Things that not sure if should be implemented at runtime level or user level.
    - Serialization / Marshalling
        - This should either be implemented at the runtime level or user/std level
        - But this should be powerful and allow for interesting use cases like python pickles
        - Write about why this is needed, and how can this be used, and how will this be implemented
    - Regexp
        - Should this be implemented in user land?
        - https://nodejs.org/en/docs/guides/dont-block-the-event-loop/#blocking-the-event-loop-redos
        - https://owasp.org/www-community/attacks/Regular_expression_Denial_of_Service_-_ReDoS
    - Crypto
        - Crypto math functions
- Lazy evaluation
- Proper definition of the Spread syntax
- bigints
    - Should this be provided as part of the language semantic like python numbers?
    - Or as a standard library implemented in SS?
    - Or as a standard library implemented in Rust then linked via FFI / dynamic library linking / running the code seperatly and calling the process?
- SIMD support
    - Will this be directly exposed to the user or implemented in the underlying executable?
- Permissions model like ink and deno
    - Where you can specify what permissions to give untrusted scripts, effectively limiting their control and sandboxing them
    - https://github.com/thesephist/ink#isolation-and-permissions-model
- Pointers
    - https://golang.org/doc/faq#no_pointer_arithmetic
    - Probably no pointers needed since the goal is to simplify things

## Preferences
- Use camelCase for value and function names
- Tabs over spaces, because it is quicker to tokenize in simple scanner implementations
