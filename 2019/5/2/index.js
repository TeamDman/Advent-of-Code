const fs = require("fs");

const origin = fs.readFileSync("input.txt","utf8")
    .replace(/\n/g,'')
    .split(",")
    .map(x => parseInt(x));
    
function attempt(suppliedInput) {
    const list = origin.slice();
    const output = [];
    let index = 0;
    while (true) {
        let block = list[index]+"";
        let op = block.slice(block.length-2);
        let modes = block.slice(0,block.length-2).split("").reverse();
        let a, aa, b, bb, c, cc;
        a = list[index+1] || 0;
        b = list[index+2] || 0;
        c = list[index+3] || 0;
        if (modes[0]!=1)
            aa = list[a] || 0;
        else
            aa = a;
        if (modes[1]!=1)
            bb = list[b] || 0;
        else
            bb = b;
        if (modes[2]!=1)
            cc = list[c] || 0;
        else
            cc = c;
        console.log(list.slice(index,index+4));
        if (op == 1) {
            console.log(`$1 [${c}]<=${aa}+${bb}`);
            list[c]=aa+bb;
            index += 4;
        } else if (op == 2) {
            console.log(`$2 [${c}]<=${aa}*${bb}`);
            list[c]=aa*bb;
            index += 4;
        } else if (op == 3) {
            console.log(`$3 [${a}]<=${suppliedInput}`);
            list[a] = suppliedInput;
            index += 2;
        } else if (op == 4) {
            console.log(`$4 [${a}]=> ${aa}`);
            output.push(list[a]);
            index += 2;
        } else if (op == 5){
            console.log(`$5 JMP ${aa}==true ? GOTO ${bb}`);
            if (aa!=0) {
                index = bb;
            } else {
                index += 3;
            }
        } else if (op == 6) {
            console.log(`$6 JMP ${aa}==false ? GOTO ${bb}`);
            if (aa==0) {
                index = bb;
            } else {
                index += 3;
            }
        } else if (op == 7) {
            console.log(`$7 [${c}] <= ${aa} LT ${bb}`);
            list[c] = aa < bb ? 1 : 0;
            index+=4;
        } else if (op == 8) {
            console.log(`$8 [${c}] <= ${aa} EQ ${bb}`);
            list[c] = aa == bb ? 1 : 0;
            index+=4;
        } else if (op == 99) {
            console.log(`$99 BREAK`);
            return output;
        } else {
            throw `Bad opcode <${op}> from block [${block}] at position ${index}`;
        }
    }  
}

console.dir(attempt(5));