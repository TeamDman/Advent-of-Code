input = document.querySelector("pre").innerText
	.trim()
	.split(",")
	.map(v => parseInt(v));

fish = {};

day = 0;

function addFish(fish, i,v) {
	fish[i] = fish[i] ?? 0;
	fish[i] += v;
}

input.forEach(v => addFish(fish, v, 1));

function breed() {
	newFish = {};
	for(j=8; j>=0; j--) {
		if (j === 0) {
			addFish(newFish, 8, fish[j]??0);
			addFish(newFish, 6, fish[j]??0);
		} else {
			addFish(newFish, j-1, fish[j]??0);
		}
	}
	fish = newFish;
}

function countFish() {
	return Object.entries(fish).reduce((prev, [k,v]) => prev+v, 0);
}



for (i=0; i<5; i++) breed();
console.log("total fish 5",countFish());
for (; i<80; i++) breed();
console.log("total fish 80",countFish());
for (; i<256; i++) breed();
console.log("total fish 256", countFish());