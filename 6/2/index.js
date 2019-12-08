const fs = require("fs");
const lines = fs.readFileSync("input.txt","utf8")
    .split("\n")
    .map(l => ({inner:l.slice(0,l.indexOf(")")),outer:l.slice(l.indexOf(")")+1)}))

function getKids(node, visited={}) {
    const rtn = [];
    for (const {inner, outer} of lines) {
        if (inner === node) {
            if (visited[outer] === undefined) {
                visited[outer] = true;
                rtn.push(outer, ...getKids(outer, visited));
            }
        }
    }
    return rtn;
}

function distanceTo(node, term, depth=1) {
    for (const {inner, outer} of lines) {
        if (inner === node) {
            if (outer === term)
                return depth;
            const d = distanceTo(outer, term, depth+1);
            if (d !== 0)
                return d;
        }
    }
    return 0;
}

const root = getKids("COM")
    .map(x=>({key: x, kids: getKids(x)}))
    .filter(x => x.kids.some(x => x === "YOU"))
    .filter(x => x.kids.some(x => x === "SAN"))       
    .sort((a,b) => a.kids.length-b.kids.length)
    .map(x => x.key)
    [0];
console.log(root);
const youDist = distanceTo(root, "YOU");
const sanDist = distanceTo(root, "SAN");
console.log(`Distance to Santa's planet: ${youDist+sanDist-2}`);