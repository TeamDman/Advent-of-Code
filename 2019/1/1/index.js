const fs = require("fs");

const input = fs.readFileSync("input.txt", "utf8");
console.dir(input);
let total = 0;
for (const line of input.split("\n")) {
    let x = parseInt(line);
    if (isNaN(x))
        break;
    x = Math.floor(x/3)-2;
    console.log(x);
    total += x;
}
console.log("done");
console.log(total);
