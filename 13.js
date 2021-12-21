var content = document.querySelector("pre").innerText;

// content=`6,10
// 0,14
// 9,10
// 0,3
// 10,4
// 4,11
// 6,0
// 6,12
// 4,1
// 0,13
// 10,12
// 3,4
// 3,0
// 8,4
// 1,10
// 2,14
// 8,10
// 9,0

// fold along y=7
// fold along x=5`;

var input = content
    .trim()
    .split("\n\n")
    .map(block => block.split("\n"));

var dots = [];
for (const line of input[0]) {
    var [x, y] = line.split(",").map(v => parseInt(v, 10));
    dots.push([x,y]);
}

function unique(dots) {
    var dict = {};
    for (const [x,y] of dots) {
        dict[x] = dict[x] ?? {};
        dict[x][y] = true;
    }
    var rtn = [];
    for (const x of Object.keys(dict)) {
        for (const y of Object.keys(dict[x])) {
            rtn.push([parseInt(x),parseInt(y)]);
        }
    }
    return rtn;
}

function max(list) {
    return list.reduce((acc, v) => Math.max(acc, v), 0);
}

function disp(dots) {
    var maxX = max(dots.map(v => v[0])) + 1;
    var maxY = max(dots.map(v => v[1])) + 1;
    grid = [...new Array(maxY)].map(row => [...new Array(maxX)].map(_ => "."));
    for (const [x,y] of dots) {
        grid[y][x] = "#";
    }
    grid = grid.map(row => row.join("")).join("\n");
    console.log(grid);
}

var folds = [];
var i=0;
for (const line of input[1]) {
    var [_, axis, value] = line.match(/(x|y)=(\d+)/);
    value = parseInt(value, 10);
    console.log("folding along",axis,value, line);
    newDots = [];
    while (dots.length > 0) {
        var dot = dots.pop();
        if (axis=="x") {
            if (dot[0] > value) {
                diff = dot[0] - value;
                dot[0] = value - diff;
            }
        } else {
            if (dot[1] > value) {
                diff = dot[1] - value;
                dot[1] = value - diff;
            }
        }
        newDots.push(dot);
    }
    dots=unique(newDots);
    if (i==0)
        console.log("part1", dots.length);
    i++;
}

disp(dots);