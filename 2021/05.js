input = document.querySelector("pre").innerText
	.trim()
	.split("\n");

pairs = input.map(line => line.split("->").map(v => v.split(",").map(k => parseInt(k))));

max = pairs.reduce(
	(acc, cur) => Math.max(acc, cur.reduce(
		(acc, cur) => Math.max(acc, cur.reduce(
			(acc, cur) => Math.max(acc, cur),0
		)), 0
	)), 0
);

function eq(a,b) {
	return a[0]===b[0] && a[1]===b[1];
}

function add(a,b) {
	return [a[0]+b[0], a[1]+b[1]];
}

function neg(a) {
	return [-a[0], -a[1]];
}

function norm(a) {
	return [Math.sign(a[0]), Math.sign(a[1])];
}

function getPointsBetween(start, end) {
	diff = norm(add(end, neg(start)));
	rtn = [];
	current = start;
	i=0;
	while (!eq(current, end)) {
		if (i++ > max+2) {
			throw new Exception("inf loop");
		}
		rtn.push(current);
		current = add(current, diff);
	}
	rtn.push(end);
	return rtn;
}

function getCount(pairs) {
	count = {};
	grid = [...new Array(max+1)].map(_ => [...new Array(max+1)].map(_=>0));
	for (const [start, end] of pairs) {
		z = getPointsBetween(start, end);
		for (const point of z) {
			prev = grid[point[0]][point[1]];
			
			if (count[prev] === undefined) count[prev]=0;
			if (count[prev+1]===undefined) count[prev+1]=0;
	
			if (prev > 0 && count[prev] > 0) count[prev]--;
			count[prev+1]++;
	
			grid[point[0]][point[1]]++;
		}
	}
	return count;
}


function getIntersecting(count) {
	return Object.entries(count)
	.filter(([key, val]) => key >= 2)
	.reduce((prev, [key, val]) => prev += val, 0);
}

console.log("max", max)

count = getCount(pairs.filter(pair => pair[0][0] == pair[1][0] || pair[0][1] == pair[1][1]));
maxVal = getIntersecting(count);
console.log("horiz", maxVal);

count = getCount(pairs);
maxVal = getIntersecting(count);
console.log("all", maxVal);