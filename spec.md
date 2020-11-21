# SS (SimpleScript)
- Simple
- Quick to learn
- No assumptions / quirkiness

idiot n beginner proof language
prevent me from saying im so dumb when i wastedays trying to figure smth out because it is not intuitive
simple, flexible and idiot proof n practical

Technical features:
- Statically typed
    - Need to know the type at Compile time if compiled
    - Need to know the 
    - actually the code is always "compiled" first into a Middle IR that is simple a token, that can be parsed later on.
        - so in this case, even for the interpreter version, the Type rule is also enforced
- Immutable
    - no data can be changed once created.
- Functional
- 

## Language Features
- Can be both intepreted and compiled AOT into an executable
    - And obviously they should work the exact same way, just one faster and native to the platform
    - If compiled, any constants defined at compile time, will be preprocessed to replace the values directly in the code? or will LLVM take care of this?
    - We also need to take care of cross compilation techniques.
- Focus on explicit representation of ideas via code. Instead of like JS where there are alot of assumptions/quirks/implicit behaviours/magic
- Comes with a GC, will be explored further below
- Paradigm
    - Procedural
    - Functional
    - Reactive???
    - Metaprogramming
- Expressive and extensible using metaprogramming concepts
- Inspiration
    - Javascript / Typescript / Rust / 
- Package management like npm, allow user to pass in a hash for a module, so that when downloading, the tool should verify it...




## Build
- Compile using LLVM backend
- Execute using Rust/Other intepreters
    - Might support JIT integrations, but... tbd
- Support transpilation options? Like transpile to TS/JS/Rust

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
If I write the code on a 64bit x86 platform, it should perform the SAME exact way on a 32bit RISC platform.

## Data** types and structures and Value declaration
- No variables
- All values are constants (IMMUTABLE)
    - Note that there is no way of declaring variables, you can only create new constants
- Strongly typed language
- Should we enforce implicit typing in all places? Or can we have type inference??
    - esp needed for things like getting a value out from a object
    - but if all the structs have fixed schema, shouldnt we be able to know the type too?
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
### Special data types
- Object
    - key value maps
    - {}
- struct
    - Objects with fixed schema
    - whereas objects just random KV maps
- Array
    - Can arrays be expanded? Or are they like rust tuples with fixed length?
    - since arrays are hard length
        - can we check n prevent out of bounds error?
        - [1, 2][4] --> invalid
    - []
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
- SS will come with a GC, either with the runtime or part of the compiled executable

## Operators
### logical
When executing expressions with logical operations "and" + "or" short circuting will be applied.
- not
- !
- and
- &&
- or
- ||

### binary
- ~
- &
- |

### Math
- +
- -
- *
- /
- %
- ^ // Should we include this?
- Notice that they are no Increment and Decrement operators
    - Say you see some nasty code like ```---a;```
    - Is it valid? That depends on how the scanner splits the lexemes. If the scanner sees it like: ```- --a;```
    - Then it could be parsed. But that would require the scanner to know about the grammatical structure of the surrounding code, which entangles things more than we want. Instead, the maximal munch rule says that it is always scanned like: ```-- -a;```
    - It scans it that way even though doing so leads to a syntax error later in the parser.
    - Similiar to rust and lox

### Comparison
- ==
- !=
- >
- <
- >=
- <=

## Scope
- Block scope
{
    
}
- Function / conditionals (IfElse) /Loop scopes are all block scopes
- variable inheritance in scope
- child scope can always access things in the outer scope
    - however parent scope cannot access things in child scope
    - only upward access
    

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


## Control flows
### If/ElseIf/Else
```js
if (condition) {
    
} else if (condition) {
    
} else {
    
}

if (condition) {
    
}
else if (condition) {
    
}
else
{
    
}
```

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


## Functions
- Split into pure and impure functions
    - Uses decorators
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
- should functions be hoisted? or cannot be accessed after definition
- overloading?
- Should there be implicit returns for functions? Does that mean we need to support undefined?
- Named function arguments? Removes the need for overloading and undefined function inputs to pass in a argument later in the sequence
- C style syntax uses ()
    - but when we do something like nested functions, there end up with too man braces, thus making the elm syntax much much nicer
    - instead of f1(f2(f3(f4(f5(arg)))))
    - elm syntax f1 f2 f3 f4 f5 arg

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
@impure // Use decorators to implicitly declare that these are impure functions (anything with side effect / not pure input/output type)
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
> tl;dr  
Single Threaded code just like JavaScript, but users can use the provided event loop library from std lib.  
Supports multi process code by spinning up new native os processes (implemented by os not our runtime).  
For now, no kernel thread support, rather user level thread via thread libraries from std lib.

- Similiar to how JavaScript uses a event based / reactive paradigm to achieve asynchronous programming and concurrency, SimpleScript supports asynchronous programming natively making it as easy as possible for users to do asynchronous programming.
- However the problem with JavaScript's approach is that it limits what the user is able to create because of its single threaded nature. And although JS supports web workers and Node JS supports seperate processes, it is not as ergonomic to the user and can be really confusing even for experienced users, as the Single Threaded nature and event based design can often screw with our existing understanding of how seperate threads and processes work.
    - Not to mention that this makes the language implementation more difficult.
- For newer JS users, async/concurrency concepts are also harder to understand, and more often then not, they have to learn the quirks of the event loop the hard way, which is often painful and time consuming (not simple at all 😫)
- Thus the approach taken by SimpleScript is to combine the best of most worlds as described in the tl;dr by focusing on providing a great bare minimum setup with simple ways to extend it.


## Modules
- Support breaking code up into modules. Every new file is a module

### Import
- @todo instead of std:libraryname should be std/libraryName?
- Import a module from standard library
    ```js
    import moduleName from "std:libraryName"
    ```
- Import a module from a library
    ```js
    import moduleName from "moduleLocation"
    ```
- Import a module using relative path
    ```js
    import moduleName from "./myModuleName"
    ```
- Import a module into its own namespace instead of the current module's namespace
    ```js
    import "./myModuleName" as moduleName
    ```
- Libraries/Modules can be scoped in packages
    - e.g. many libraries/modules inside the standard library package
    - namespaced / scoped with std:
    - you can have nested : scoping
    - scoping should be based on file/dir structure

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


## Language extensions
### FFI
- Will have builtin language/std-lib level support for FFI to interact with Rust and C/C++ code in the future


## Preferences
- Use camelCase for constant values