var content = document.querySelector("pre").innerText;

// content=`NNCB

// CH -> B
// HH -> N
// CB -> H
// NH -> C
// HB -> C
// HC -> B
// HN -> C
// NN -> C
// BH -> H
// NC -> B
// NB -> B
// BN -> B
// BB -> N
// BC -> B
// CC -> N
// CN -> C`;

var [input, rules] = content
    .trim()
    .split("\n\n")
    .map(block => block.split("\n"));
input = input[0].split("");

rules = rules.reduce((map, cur) => {
    var [left, right] = cur.split(" -> ");
    map[left] = right;
    return map;
}, {});

function inc(input, key, amount=1) {
    input[key] = (input[key] ?? 0) + amount;
}

function getBigrams(input) {
    rtn = {};
    left = input.shift();
    right = input.shift();
    while (right !== undefined) {
        inc(rtn, left+right);
        left = right;
        right = input.shift();
    }
    return rtn;
}
input=getBigrams(input);

function step() {
    var next = {};
    for (const key in input) {
        var val = rules[key];
        var amt = input[key];
        if (val === undefined) {
            inc(next, key, amt);
            continue;
        }
        var [left, right] = key.split("");
        inc(next, left+val, amt);
        inc(next, val+right, amt);
    }
    input = next;
}

function bigrams2counts(input) {
    return Object.entries(
            Object.entries(input)
                .flatMap(([k,v]) => k.split("").map(x => [x,v]))
                .reduce((map, [k,v]) => {
                    inc(map, k, v);
                    return map;
                }, {})
        )
        .map(([k,v]) => [k,Math.ceil(v/2)]);
}

function getResult(input) {
    least = [Number.MAX_SAFE_INTEGER];
    most = [0];
    for (const [k,v] of bigrams2counts(input)) {
        if (v < least[0]) {
            least = [v,k];
        }
        if (v > most[0]) {
            most = [v,k];
        }
    }
    console.log(least, most);
    return most[0]-least[0];
}

console.log(input);

// step();
// step();
// step();
// console.log(JSON.stringify(input) == JSON.stringify(getBigrams("NBBBCNCCNBBNBNBBCHBHHBCHB".split(""))));
// console.log(input);

for(var i=0; i<40; i++){
    if (i==10) {
        console.log("part1", getResult(input));
    }
    step();
}
console.log("part2", getResult(input));
// too low 2188189693529