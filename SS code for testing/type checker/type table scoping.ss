/* Test of type table scoping */

// Define outer test value as type Number
const test = 5;

{
  // Define inner test value as type String
  const test = "a";
  print test;
}

/*
  This should type check, since outer test value is type Number
  However if the type checker did not handle scoping change of block scopes,
  then this will fail type checking, as the type will be read as String instead
*/
print test + 7;