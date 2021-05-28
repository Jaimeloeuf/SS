# Simply Typed Lambda Calculus
Experiment building a typed purely functional language based on <https://blog.mgechev.com/2017/08/05/typed-lambda-calculus-create-type-checker-transpiler-compiler-javascript/>

Originally described as: A type checker and interpreter for simply typed lambda calculus written in JavaScript.


## Example
The following program:
```
(λ a: Int → a) if (λ a: Int → iszero a) pred 0 then succ 0 else 0
```
Is correct and evaluates to `0`.

### Run the code with the interpreter
```
$ npm run run demo/correct1.lambda

0
```

### Transpile the code to JS
```
$ npm run compile demo/correct1.lambda

(function(a) {
  return a;
})(
  (function(a) {
    return a === 0;
  })(0 - 1)
    ? 0 + 1
    : 0
);
```


## Commits
Commits for this subrepo are prefixed with **[stlc]**

## Credits
- Credits goes to Minko Gechev <mgechev@gmail.com> for writing the original code and tutorial.
- Original code can be found here <https://github.com/mgechev/typed-calc>