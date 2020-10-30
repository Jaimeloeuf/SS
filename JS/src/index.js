const readline = require("readline");

// const rl = readline.createInterface({
//   input: process.stdin,
//   output: process.stdout,
// });

// rl.on("line", (line) => {
//   switch (line.trim()) {
//     case "hello":
//       console.log("world!");
//       break;
//     default:
//       console.log(`Say what? I might have heard '${line.trim()}'`);
//       break;
//   }
// }).on("close", () => {
//   console.log("Have a great day!");
//   process.exit(0);
// });

const path = require("path");
const fs = require("fs").promises;

// const keywords = {'const'};
const types = {};

(async function () {
  // resolve reads from this location
  // So if read from the point of execution, just use the location without the __dirname
  //   await fs.readFile(path.resolve("../../test.sl"));
  const source = await fs.readFile(
    path.resolve(__dirname, "../../test.sl"),
    "utf-8"
  );

  console.log(source);
})();
