x=0;
y=0;

input = document.querySelector("pre").innerText
	.trim()
	.split("\n");

for (const line of input) {
	let [command, amount] = line.split(" ");
	amount = parseInt(amount);
	if (command === "forward") x += amount;
	if (command === "down") y += amount;
	if (command === "up") y -= amount;
}
console.log(`forward=${x} down=${y} result=${x*y}`);

x=0;
y=0;
aim=0;
for (const line of input) {
	let [command, amount] = line.split(" ");
	amount = parseInt(amount);
	if (command === "forward") {
		x += amount;
		y += aim * amount;
	}
	if (command === "down") aim += amount;
	if (command === "up") aim -= amount;
}
console.log(`forward=${x} down=${y} result=${x*y}`);