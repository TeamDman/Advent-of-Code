const fs = require("fs");

const lines = fs.readFileSync("input.txt","utf8").split("\n");

const first = lines[0].split(",");
const second = lines[1].split(",");

const mat = [];
function set(x,y, fill) {
    if (mat[x] === undefined)
        mat[x] = [];
    if (mat[x][y] === undefined)
        mat[x][y] = fill;
    if (mat[x][y] !== fill)
        mat[x][y] = '!';
}

function drawLine(line, id) {
    let x=0, y=0;
    function step(input) {
        const amount = parseInt(input.slice(1));
        switch(input[0]) {
            case "U":
                for (let i=1; i<=amount; i++) 
                    set(x,y+i, id);
                y+=amount;
                break;
            case "D":
                for (let i=1; i<=amount; i++)
                    set(x, y-i, id);
                y-=amount;
                break;
            case "L":
                for (let i=1; i<=amount; i++)
                    set(x-i, y, id);
                x-=amount;
                break;
            case "R":
                for (let i=1; i<=amount; i++)
                    set(x+i, y, id);
                x+=amount;
                break;
        }
    }     
    for (const d of line) {
        step(d);
    }
}
drawLine(first,'a');
drawLine(second,'b');
let dist = -1;
for (let i of Object.keys(mat)) {
    for (let j of Object.keys(mat[i])) {
        if (mat[i][j] === '!') {
            const d = Math.abs(parseInt(i)) + Math.abs(parseInt(j));
            if (dist === -1 || d < dist)
                dist = d;
        }
    }
}
console.log(`Distance=${dist}`);