# Type Checker
Type Checker to do type checking with type inference solely based on how the expressions are used  
Based on the experiment building [stlc](../../../Simply%20Typed%20Lambda%20Calculus)

## Details
I realise that I basically stumbled upon what is basically a "version" of the Hindley Milner type inference method without type unification.

HMR type checking is a general type inference approach
- Infers the types of constructs that are not explicitly declared
- It does so by leveraging the constraints of various constructs (if stmt, must have bool conditionals)
- It then apples these constraints together with type unification, to find the most general type for each construct, or its a type error if there is no type general enough to satisfy the constraints

This current method does everything except for the last point of type unification

Instead of having type variables, this uses Type::Lazy and universal equality plus re-check during function call to almost achieve the same thing
But this is much slower and not as space efficient