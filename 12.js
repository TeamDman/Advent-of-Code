var content = document.querySelector("pre").innerText;

// content = `start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end`; // 10

// content = `dc-end
// HN-start
// start-kj
// dc-start
// dc-HN
// LN-dc
// HN-end
// kj-sa
// kj-HN
// kj-dc`; // 19

// content = `fs-end
// he-DX
// fs-he
// start-DX
// pj-DX
// end-zg
// zg-sl
// zg-pj
// pj-he
// RW-he
// fs-DX
// pj-RW
// zg-RW
// start-pj
// he-WI
// zg-he
// pj-fs
// start-RW`;

var input = content
    .trim()
    .split("\n");

var transitions = {};

for (const line of input) {
    var [left, right] = line.split("-");
    transitions[left] = transitions[left] ?? [];
    transitions[left].push(right);
    
    transitions[right] = transitions[right] ?? [];
    transitions[right].push(left);
}

var paths = [];

function getNext(path) {
    latest = path[path.length-1];
    if (latest == "end") {
        paths.push(path);
        return;
    }
    if (!isGood(path, 2)) return;
    for (const dest of transitions?.[latest] ?? []) {
        // console.log("checking", dest, path);
        if (dest == "start") continue;
        if (dest.toLowerCase() == dest && path.filter(x => x == dest).length >= 2) {
            // console.log("skipping", dest, path);
            continue;
        }
        next = path.slice();
        next.push(dest);
        getNext(next);
    }
}

getNext(["start"]);

console.log(transitions);
console.log(paths);

function isGood(path, limit) {
    return Object.values(
        path
            .filter(x => x!=="start" && x!=="end")
            .filter(x => x.toLowerCase() == x)
            .reduce((acc, x) => {acc[x] = (acc[x]??0)+1; return acc}, {})
    ).reduce((acc, cur) => cur>=limit?acc+1:acc, 0) <= 1;
}

console.log("part1", paths.filter(path => isGood(path, 1)).map(p => p.join(",")));
console.log("part2", paths.filter(path => isGood(path, 2)).map(p => p.join(",")));