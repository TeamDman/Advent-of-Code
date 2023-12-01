const fs = require("fs");
const lines = fs.readFileSync("input.txt","utf8")
    .split("\n")
    .map(l => ({inner: l.slice(0,3), outer: l.slice(4)}));

function getChildren(node, state) {
    for (const {inner, outer} of lines) {
        if (inner === node) {
            state.total+=state.depth;
            const s = {total: 0, depth: state.depth+1};
            getChildren(outer, s);
            state.total += s.total;
        }
    }
}
const state = {total:0, depth: 1};
getChildren("COM", state);
console.dir(state);