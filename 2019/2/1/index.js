const fs = require("fs");

const list = fs.readFileSync("input.txt","utf8")
    .replace(/\n/g,'')
    .split(",")
    .map(x => parseInt(x));
    
list[1]=12;
list[2]=2;

let index = 0;
while (true) {
    let op = list[index];
    if (op === 1) {
        list[list[index+3]]=(list[list[index+1]]||0)+(list[list[index+2]]||0);
    } else if (op === 2) {
        list[list[index+3]]=(list[list[index+1]]||0)*(list[list[index+2]]||0);
    } else if (op === 99) {
        console.log("Ending!");
        break;
    } else {
        console.log("Something went wrong.");
        break;
    }
    console.log(`[${index}] ${list[index]} ${list[index+1]} ${list[index+2]} ${list[index+3]} = ${list[list[index+3]]}`);
    index+=4;
}

console.dir(list[0]);