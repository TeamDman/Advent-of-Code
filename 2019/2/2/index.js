const fs = require("fs");

const origin = fs.readFileSync("input.txt","utf8")
    .replace(/\n/g,'')
    .split(",")
    .map(x => parseInt(x));
    
function attempt(noun, verb) {
    const list = origin.slice();
    list[1]=noun;
    list[2]=verb;
    
    let index = 0;
    while (true) {
        let op = list[index];
        if (op === 1) {
            list[list[index+3]]=(list[list[index+1]]||0)+(list[list[index+2]]||0);
        } else if (op === 2) {
            list[list[index+3]]=(list[list[index+1]]||0)*(list[list[index+2]]||0);
        } else if (op === 99) {
            return list[0];
        } else {
            return -1;
        }
        index+=4;
    }  
}
let i =0, j = 0, res = 0;
const target = 19690720;
let scale = 1, step=100;
while (true) {
    for (let i=step*(scale-1); i<step*scale; i++) {
        for (let j=step*(scale-1); j<step*scale; j++) {
            let x = attempt(i,j); 
            if (x === target) {
                console.log(`noun=${i}, verb=${j}, x=${x}, ans=${100*i+j}`);
            }
        }
    }
    // console.log(`scale=${scale}`);
    scale++;
}