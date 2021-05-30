const { SymbolTableImpl, Scope } = require("./symbol-table");
const { ASTNodes } = require("./ast");

const SymbolTable = new SymbolTableImpl();

const Eval = (ast) => {
  // The empty program evaluates to null.
  if (!ast) return null;

  switch (ast.type) {
    // The literals evaluate to their values
    case ASTNodes.Literal:
      return ast.value;

    // The variables evaluate to the values that are bound to them in the SymbolTable.
    case ASTNodes.Identifier:
      return SymbolTable.lookup(ast.name);

    // if-then-else evaluates to the expression of then clause if condition is true, else the value of else clause
    case ASTNodes.Condition:
      if (Eval(ast.condition)) return Eval(ast.then);
      else return Eval(ast.el);

    // The abstraction creates a new context of execution and registers it's argument in the SymbolTable.
    case ASTNodes.Abstraction: {
      const scope = new Scope();
      return (value) => {
        scope.add(ast.arg.id.name, value);
        SymbolTable.push(scope);
        return Eval(ast.body);
      };
    }

    // IsZero checks if the evaluated value of its expression equals `0`.
    case ASTNodes.IsZero:
      return Eval(ast.expression) === 0;

    // The arithmetic operations manipulate the value of their corresponding expressions:
    // - succ: adds 1.
    // - pred: substracts 1.
    case ASTNodes.Arithmetic: {
      const op = ast.operator;
      const val = Eval(ast.expression);
      switch (op) {
        case "succ":
          return val + 1;
        case "pred":
          return val === 0 ? val : val - 1;
        default:
          console.log("ERROR Unknown arithmetic operator: ", op);
          return "";
      }
    }

    // The application evaluates to:
    // - Evaluation of the left expression.
    // - Evaluation of the right expression.
    // - Application of the evaluation of the left expression over the evaluated right expression.
    case ASTNodes.Application: {
      const l = Eval(ast.left);
      const r = Eval(ast.right);
      return l(r);
    }

    default:
      console.log("Unknown AST Type: ", ast.type);
      process.exit(1);
  }
};

module.exports.Eval = Eval;
