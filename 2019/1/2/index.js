const fs = require("fs");
const lines = fs.readFileSync("input.txt","utf8").split("\n");

let total = 0;
function getCost(mass) {
    const cost = Math.floor(mass/3)-2;
    if (cost <= 0)
        return 0;
    return cost + getCost(cost);
}

for (const line of lines) {
    let x = parseInt(line);
    if (isNaN(x))
        break;
    total += getCost(x);
}

console.log(total);