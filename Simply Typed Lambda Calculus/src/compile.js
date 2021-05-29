const prettier = require("prettier");
const { ASTNodes } = require("./ast");

// Doesn't support lazy evaluation
// so examples like `demo/correct4.lambda` will not be transpiled correctly.
const CompileJS = (ast) => {
  // Empty AST compiles to an empty program ''
  if (!ast) return "";

  switch (ast.type) {
    // The literals compile to their values.
    case ASTNodes.Literal:
      return ast.value;

    // The variables compile to the identifiers.
    case ASTNodes.Identifier:
      return ast.name;

    // if-then-else compiles to a ternary expression
    case ASTNodes.Condition: {
      const targetCondition = CompileJS(ast.condition);
      const targetThen = CompileJS(ast.then);
      const targetElse = CompileJS(ast.el);
      return `${targetCondition} ? ${targetThen} : ${targetElse}\n`;
    }

    // The abstraction compiles to a function in the target language.
    case ASTNodes.Abstraction:
      return `(${ast.arg.id.name} => {
      return ${CompileJS(ast.body)}
    })`;

    // IsZero checks if the evaluated value of its expression equals `0`.
    case ASTNodes.IsZero:
      return `${CompileJS(ast.expression)} === 0\n`;

    // The arithmetic operations manipulate the value of their corresponding expressions:
    // - succ: adds 1.
    // - pred: substracts 1.
    case ASTNodes.Arithmetic: {
      const op = ast.operator;
      const val = CompileJS(ast.expression);
      switch (op) {
        case "succ":
          return `${val} + 1\n`;
        case "pred":
          return `(${val} - 1 >= 0) ? ${val} - 1 : 0\n`;

        default:
          console.log("ERROR Unknown opcode: ", op);
          return "";
      }
    }

    // The application compiles to:
    // Invocation of the compiled left expression over
    // the compiled right expression.
    case ASTNodes.Application: {
      const l = CompileJS(ast.left);
      const r = CompileJS(ast.right);
      return `${l}(${r})\n`;
    }

    default:
      console.log("Unknown AST Type: ", ast.type);
      process.exit(1);
  }
};

// Allow user to specify file path to write to
module.exports.CompileJS = (ast) =>
  prettier.format(CompileJS(ast), {
    printWidth: 80,
    tabWidth: 2,
    trailingComma: "none",
    bracketSpacing: true,
    parser: "babel",
  });
