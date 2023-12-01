delete print; // I keep typing this by mistake :/

/* Part 1 problem formulation

Given a list of integers, determine the count of values which are greater than the previous value.

*/

// Get list of integers
input = document.querySelector("pre").innerText
	.trim()
	.split("\n")
	.map(x => parseInt(x));

function countGreater(list) {
	list = list.map((value, index) => value > list?.[index-1] ?? value)
	console.log("greaters", list);
	list = list.filter(value => value)
	console.log("greaters filtered", list)
	return list.length;
}

part1 = countGreater(input);
console.log(`Part 1: ${part1}`);

/* Part 2 problem formulation

Given a list of integers:
- Compute a 3-width sliding window (index, index+1, index+2); current and next 2 values
- Compute the sum of each window
- Determine the count of sums which are greater than the previous sum

*/

function getWindows(list, windowSize) {
	return list.map((value, index) => list.slice(index, index+windowSize));
}

function sum(list) {
	return list.reduce((prev, cur) => prev+cur, 0);
}

part2 = getWindows(input, 3);
console.log("windows", part2);

part2 = part2.map(sum);
console.log("summed windows", part2);

part2 = countGreater(part2);
console.log("Part 2: creater count", part2);