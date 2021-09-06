const { SymbolTableImpl, Scope } = require("./symbol-table");
const { ASTNodes, Types } = require("./ast");

const SymbolTable = new SymbolTableImpl();

// Check if the 2 given types are the same
function typeEq(a, b) {
  // If both are arrays, it means that both are functions types
  if (a instanceof Array && b instanceof Array) {
    // Check if the length/depth of the functions/abstractions are the same, if not same, means diff type
    if (a.length !== b.length) return false;

    // Check every type of the functions/abstractions individually and recursively to ensure that they are all the same type
    // If any type in the array is different, then not equal types
    for (let i = 0; i < a.length; i += 1) if (!typeEq(a[i], b[i])) return false;

    // If all the same, then the functions/abstractions are of the same type
    return true;
  }

  // If both values are type info encoded as a string, then compare them directly
  // This means that they are both primitive types
  if (typeof a === "string" && typeof b === "string") return a === b;

  // If one value is function type and the other is primitive, then they are automatically different types
  // This is the default type equality value, where all cases that are not tested for or skipped are of unequal type
  return false;
}

// Check method which is returned, and used recursively to travers the AST
module.exports.Check = function Check(ast, diagnostics = []) {
  // By definition, an empty AST is correct
  if (!ast) return { diagnostics };

  switch (ast.type) {
    // Literals:
    // - 0 is of type Natural
    // - false and true are of type Boolean
    // Everything else is incorrect.
    case ASTNodes.Literal: {
      if (ast.value === 0)
        return {
          diagnostics,
          type: Types.Natural,
        };
      else if (ast.value === false || ast.value === true)
        return {
          diagnostics,
          type: Types.Boolean,
        };
      else {
        diagnostics.push("Unknown type literal");
        return { diagnostics };
      }
    }

    // Get the type of identifier from the symbol table
    case ASTNodes.Identifier:
      return {
        diagnostics,
        type: SymbolTable.lookup(ast.name),
      };

    // if-then-else block is correct if:
    // - The condition is of type Boolean.
    // - The expressions of 'then' and 'else' are of the same type.
    case ASTNodes.Condition: {
      if (!ast.then || !ast.el || !ast.condition) {
        diagnostics.push("No condition for a conditional expression");
        return { diagnostics };
      }

      // Boolean type check for condition
      const c = Check(ast.condition);
      diagnostics = diagnostics.concat(c.diagnostics);
      const conditionType = c.type;
      if (!typeEq(conditionType, Types.Boolean)) {
        diagnostics.push("Incorrect type of condition of condition");
        return { diagnostics };
      }

      // Get type of then expression
      const thenBranch = Check(ast.then);
      diagnostics = diagnostics.concat(thenBranch.diagnostics);
      const thenBranchType = thenBranch.type;

      // Get type of else expression
      const elseBranch = Check(ast.el);
      diagnostics = diagnostics.concat(elseBranch.diagnostics);
      const elseBranchType = elseBranch.type;

      // Check if both branches are of equal type
      if (typeEq(thenBranchType, elseBranchType)) return thenBranch;
      else {
        diagnostics.push("Incorrect type of then/else branches");
        return { diagnostics };
      }
    }

    // Abstraction registers its argument in the SymbolTable
    // and returns a pair:
    // - The type of its argument.
    // - Type of its body, which may depend on the type
    // of the argument registered in the SymbolTable.
    case ASTNodes.Abstraction: {
      const scope = new Scope();
      scope.add(ast.arg.id.name, ast.arg.type);
      SymbolTable.push(scope);
      if (!ast.body) {
        diagnostics.push("Missing function body");
        return { diagnostics };
      }

      const body = Check(ast.body);
      const bodyType = body.type;
      diagnostics = diagnostics.concat(body.diagnostics);
      if (!bodyType) {
        diagnostics.push("Incorrect type of the body");
        return { diagnostics };
      }
      return {
        diagnostics,
        type: [ast.arg.type, bodyType],
      };
    }

    // The type of IsZero is Boolean but in case
    // its argument is not Natural the program is incorrect.
    case ASTNodes.IsZero: {
      const body = Check(ast.expression);
      diagnostics = diagnostics.concat(body.diagnostics);
      const bodyType = body.type;
      if (!typeEq(bodyType, Types.Natural)) {
        diagnostics.push("Incorrect type of IsZero");
        return { diagnostics };
      }

      return {
        diagnostics,
        type: Types.Boolean,
      };
    }

    // The type of the arithmetic operations are Natural
    // but in case the type of the body is not the entire
    // program is incorrect.
    case ASTNodes.Arithmetic: {
      const body = Check(ast.expression);
      diagnostics = diagnostics.concat(body.diagnostics);
      const bodyType = body.type;
      if (!typeEq(bodyType, Types.Natural)) {
        diagnostics.push(`Incorrect type of ${ast.operation}`);
        return { diagnostics };
      }
      return {
        diagnostics,
        type: Types.Natural,
      };
    }

    // The type of:
    // e1: T1, e2: T2, e1 e2: T1
    case ASTNodes.Application: {
      const l = Check(ast.left);
      // Immediately end if function type is undefined, which means error occurred in the nested left child(s)
      if (!l.type) return { diagnostics };

      const leftType = l.type;
      diagnostics = diagnostics.concat(l.diagnostics);

      const r = Check(ast.right);
      const rightType = r.type;
      diagnostics = diagnostics.concat(r.diagnostics);

      // If right child have no type, or if right child's type matches function parameter type,
      // return type as the type of the function body
      if (!ast.right || leftType[0] === rightType) {
        return {
          diagnostics,
          type: leftType[1],
        };
      } else {
        diagnostics.push("Incorrect type of application");
        return { diagnostics };
      }
    }

    default:
      // console.log("Unknown AST Type: ", ast.type);
      // process.exit(1);

      console.log("Unknown AST Type: ", ast.type);
      return { diagnostics };
  }
};
