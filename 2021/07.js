input = document.querySelector("pre").innerText
	.trim()
	.split(",")
	.map(v => parseInt(v));

crabs = input;

function getCost(target) {
	return crabs.map(v => Math.abs(target-v)).reduce((prev, v) => prev+v, 0);
}


function sum(n) {
	return (n * (n+1)) / 2;
}

function getWorseCost(target) {
	return crabs.map(v => sum(Math.abs(target-v))).reduce((prev, v) => prev+v, 0);
}

costs = crabs.map((_, i) => getCost(i));
bestCost = costs.reduce((prev, v, i) => v < costs[prev] ? i : prev, 0);
console.log(`Best target is ${bestCost} with cost ${costs[bestCost]}`);


costs = crabs.map((_, i) => getWorseCost(i));
bestCost = costs.reduce((prev, v, i) => v < costs[prev] ? i : prev, 0);
console.log(`Best target is ${bestCost} with cost ${costs[bestCost]}`);