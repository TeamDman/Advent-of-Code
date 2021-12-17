input = document.querySelector("pre").innerText
	.trim()
	.split("\n")
	.map(v => v.split(" | ").map(v => v.split(" ")));

real=["abcefg", "cf", "acdeg", "acdfg","bcdf","abdfg","abdefg","acf","abcdefg","abcdfg"];

lens = [real[1].length, real[4].length, real[7].length, real[8].length];

part1 = input.flatMap(x => x[1]).filter(x => lens.includes(x.length)).length
console.log("part1", part1);

// 
// 
// 

function includes(a,b) {
	return [...b].every(c => a.indexOf(c) !== -1);
}

function except(a, b) {
	return [...a].filter(x => !includes(b,x)).join("");
}

function union(a,b) {
	return [...a, ...b].join("");
}

function intersect(a, b) {
	return [...a].filter(x => includes(b, x)).join("");
}

function diff(a, b) {
	return except(union(a,b), intersect(a,b));
}

function eq(a,b) {
	return includes(a,b) && includes(b,a);
}

var accum = 0;
for (const line of input) {
	var [left, right] = line;

	var symbols = line.flatMap(x => x);

	var lookup = {}
	lookup[1] = symbols.find(x => x.length === 2);
	lookup[4] = symbols.find(x => x.length === 4);
	lookup[7] = symbols.find(x => x.length === 3);
	lookup[8] = symbols.find(x => x.length === 7);

	// lookup[6] = a + except(lookup[8], lookup[7]);
	lookup[3] = symbols.filter(x => x.length===5).find(x => includes(x, lookup[7]));

	var a = except(lookup[7], lookup[1]);
	var eg = except(except(lookup[8], lookup[4]), a);
	var bd = except(lookup[4], lookup[1]);
	
	var e = except(eg, lookup[3]);
	var g = except(eg, e);
	var b = except(bd, lookup[3]);
	var d = except(bd, b);

	lookup[9] = except(lookup[8], e);
	lookup[0] = except(lookup[8], d);
	lookup[5] = symbols.filter(x => x.length===5).find(x => includes(x, b) && !includes(x, e));

	var c = except(lookup[1], lookup[5]);

	lookup[6] = except(lookup[8], c);
	lookup[2] = a+c+d+e+g;

	function get(sym) {
		return Object.entries(lookup).find(([k,v]) => eq(v,sym))[0];
	}

	var tot = "";
	for (const sym of right) {
		tot+=get(sym);
	}
	accum+=parseInt(tot);
}
console.log("part2", accum);