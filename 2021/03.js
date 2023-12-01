input = document.querySelector("pre").innerText
	.trim()
	.split("\n");

function getBits(lines) {
	bits = lines[0].split("").map(_ => ({'0':0, '1':0}));
	for (const line of lines) {
		for (const i in line) {
			bits[i][line[i]]++;
		}
	}
	return bits
}
bits = getBits(input);

gamma = 0;
epsilon = 0;
console.log(bits)

function greater(dict) {
	return dict["0"] > dict["1"] ? "0" : "1";
}
function minner(dict) {
	return dict["0"] <= dict["1"] ? "0" : "1";
}

for (const i in bits) {
	gamma <<= 1;
	epsilon <<= 1;
	if (greater(bits[i]) === "0") {
		epsilon |= 1;
	} else {
		gamma |= 1;
	}
}
console.log(`gamma=${gamma.toString(2)} epsilon=${epsilon.toString(2)} result=${gamma*epsilon}`);

lines = input.slice()
i=0;
while (lines.length > 1) {
	max = greater(bits[i]);
	lines = lines.filter(x => x[i] === max)
	bits = getBits(lines);
	console.log(bits[i], i, max, lines)
	i++;
}
oxy = parseInt(lines[0], 2);
console.log("oxy", lines[0]);


lines = input.slice();
i=0;
while (lines.length > 1) {
	min = minner(bits[i]);
	lines = lines.filter(x => x[i] === min)
	bits = getBits(lines);
	console.log(bits[i], i, min, lines)
	i++;
}
co2 = parseInt(lines[0],2);
console.log("co2", lines[0]);
console.log(`oxy=${oxy} co2=${co2} result=${oxy*co2}`);