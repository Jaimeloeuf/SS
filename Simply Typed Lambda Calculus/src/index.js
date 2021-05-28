const { parse } = require("./simply-typed");
const { Check } = require("./check");
const { Eval } = require("./eval");
const { green, red } = require("chalk");
const { CompileJS } = require("./compile");
const { existsSync, readFileSync } = require("fs");

function main() {
  // Get file name from arguments
  const fileName = process.argv.pop();

  if (!existsSync(fileName)) {
    console.error(`"${fileName}" does not exist.`);
    process.exit(1);
  }

  // Get source code synchronously and parse for AST immediately
  const sourceCode = readFileSync(fileName, { encoding: "utf-8" });
  const ast = parse(sourceCode);

  // Check if there is any issues with the AST, if there is, log all errors and exit process
  const diagnostics = Check(ast).diagnostics;
  if (diagnostics.length) {
    console.error(red(diagnostics.join("\n")));
    process.exit(1);
  }

  /* Either compile or evaluate the AST next */

  // Check if there is a "compile" string arguement before the file name
  if (process.argv.pop() === "compile") {
    console.log(green(`Compiling '${fileName}' to JavaScript\n`));
    console.log(CompileJS(ast));
  }
  else {
    console.log(green(`Evaluating '${fileName}'\n`));
    console.log(Eval(ast));
  }

}

main();