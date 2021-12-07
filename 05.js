input = document.querySelector("pre").innerText
	.trim()
	.split("\n");

pairs = input.map(line => line.split("->").map(v => v.split(",").map(k => parseInt(k))));

flat = pairs.filter(pair => pair[0][0] == pair[1][0] || pair[0][1] == pair[1][1]);

max = pairs.reduce(
	(acc, cur) => Math.max(acc, cur.reduce(
		(acc, cur) => Math.max(acc, cur.reduce(
			(acc, cur) => Math.max(acc, cur),0
		)), 0
	)), 0
);
console.log(max)

grid = [...new Array(max)].map(_ => [...new Array(max)].map(_=>0));

function eq(a,b) {
	return a[0]===b[0] && a[1]===b[1];
}

function add(a,b) {
	return [a[0]+b[0], a[1]+b[1]];
}

function neg(a) {
	return [-a[0], -a[1]];
}

function badNorm(a) {
	return a[0]===0?[0,Math.sign(a[1])]:[Math.sign(a[0]),0];
}

function getPointsBetween(start, end) {
	diff = badNorm(add(end, neg(start)));
	rtn = [];
	current = start;
	while (!eq(current, end)) {
		rtn.push(current);
		current = add(current, diff);
	}
	rtn.push(end);
	return rtn;
}

count = {};

for (const [start, end] of flat) {
	z = getPointsBetween(start, end);
	for (const point of z) {
		prev = grid[point[0], point[1]];

		if (count[prev] === undefined) count[prev]=0;
		if (count[prev+1]===undefined) count[prev+1]=0;

		if (isNaN(prev)) {
			console.log(point, grid[point[0], point[1]], count);
		}
		if (prev > 0 && count[prev] > 0) count[prev]--;
		count[prev+1]++;

		grid[point[0], point[1]]++;
	}
}

maxKey = Math.max(...Object.keys(count));
maxVal = count[maxKey];
console.log(maxKey, maxVal);