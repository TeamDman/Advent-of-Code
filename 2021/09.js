// content = `2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678`;
content = document.querySelector("pre").innerText;
input = content
	.trim()
	.split("\n")
    .map(x => x.split("").map(x=>parseInt(x)));

var lows = [];

function getAdj(i,j) {
    var above = [i-1, j];
    var below = [i+1, j];
    var left = [i, j-1];
    var right = [i, j+1];
    var all = [above, below, left, right]
        .map(([i,j]) => [input[i]?.[j], [i,j]])
        .filter(x => x[0]!==undefined);
    return all;
}

for(var i=0; i<input.length; i++) {
    for (var j=0; j<input[i].length; j++) {
        var it = input[i][j];
        if (!getAdj(i,j).some(x => x[0]<=it)) {
            lows.push([it, [i, j]]);
        }
    }
}

risk = lows.reduce((prev, [it, [i, j]]) => prev+it+1, 0);
console.log("part1", risk);

function eq(a,b) {
    return a[0] == b[0] && a[1] == b[1];
}

var basins = [];
for (const low of lows) {
    var valid = [];
    var toVisit = [low];
    var visited = [];
    while (toVisit.length > 0) {
        const current = toVisit.pop();

        if (visited.some(v => eq(v, current[1]))) continue;
        visited.push(current[1]);

        if (current[0] === 9) continue;

        valid.push(current);
        for (const adj of getAdj(...current[1])) {
            // if (adj[0] === current[0]+1) { // reeeee
            // im fucking pissed I thought it had to be continuous
                toVisit.push(adj);
            // }
        }
    }
    basins.push(valid);
}
var basinSizes = basins.sort((a,b) => b.length - a.length).slice(0,3).reduce((acc, b) => acc*b.length,1);
console.log("part2", basinSizes);

function dispBasin(b) {
    var blank = input.map(row => row.map(col => " "));
    for (const [it, [i, j]] of b) {
        blank[i][j] = it;
    }
    return "\n" + blank.map(row => "|" + row.join("") + "|").join("\n");
}

console.log(dispBasin(basins[0]));
console.log(dispBasin(basins[1]));
console.log(dispBasin(basins[2]));
// 15813 low
// 305536 ???
// 187348392 high
// 34800500 high
// 856716 ans