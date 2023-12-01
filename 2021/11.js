var content = document.querySelector("pre").innerText;

// content = `5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526`;

// content=`11111
// 19991
// 19191
// 19991
// 11111`;

var input = content
    .trim()
    .split("\n")
    .map(line => line.split("").map(v => parseInt(v, 10)));

function getAdj(x, y) {
    var rtn = [];
    for (var i = -1; i <= 1; i++) {
        for (var j = -1; j <= 1; j++) {
            if (i == 0 && j==0) continue;
            rtn.push([x+i, y+j]);
        }
    }
    return rtn.filter(([i, j]) => input?.[i]?.[j] !== undefined);
}

var flashed = [];
var flashMap = {};
function flash(i,j) {
    if (input[i][j] < 10) return;
    if (flashMap?.[i]?.[j] !== undefined) return;
    flashMap[i] = flashMap[i] ?? {};
    flashMap[i][j] = true;
    flashed.push([i,j]);

    getAdj(i, j).forEach(([i, j]) => {
        input[i][j]++;
        flash(i,j);
    });
}

function hash() {
    return input.map(row => row.join(" ")).join("\n");
}

function shouldStop() {
    return input.every(row => row.every(col => col == 0));
}

var flashCount = 0;
function step() {
    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input[i].length; j++) {
            input[i][j]++;
        }
    }
    
    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input[i].length; j++) {
            if (input[i][j] > 9) {
                flash(i,j);
            }
        }
    }
    flashed.forEach(([i,j]) => input[i][j]=0);
    flashMap = {};
    flashCount += flashed.length;
    flashed = [];
}

console.log(hash(input));

// step();

for(i=0;; i++) {
    step();
    if (i==99) {
        console.log("part1", flashCount);
    }
    if (shouldStop()) {
        console.log(hash(input), i);
        console.log("part2", i+1);
        break;
    }
}