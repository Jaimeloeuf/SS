# Language Design Guidelines
Here are some guidelines to follow when adding new language features or updating the language itself.


## 1. Always prefer using lesser code for the same effect and **clarity**
For example, the keyword for function definition has been changed from `function` to `fn`

v1
```
function demo() {
    // code
}
```

v2
```
fn demo() {
    // code
}
```

Both achieves the same result, but v2 is better because there is lesser code and is easier to read with the same amount of clarity.