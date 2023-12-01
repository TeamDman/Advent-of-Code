const fs = require("fs");
const input = fs.readFileSync("./input.txt","utf8")
	.replace(/\n/g,'')
	.split(",")
	.map(x => parseInt(x));
	
function intcode(tape, suppliedInputs, debug) {
	const list = tape.slice();
	const output = [];
	let index = 0;
	while (true) {
		let block = list[index]+"";
		let op = block.slice(block.length-2);
		let modes = block.slice(0,block.length-2).split("").reverse();
		let a, aa, b, bb, c, cc;
		a = list[index+1] || 0;
		b = list[index+2] || 0;
		c = list[index+3] || 0;
		if (modes[0]!=1)
			aa = list[a] || 0;
		else
			aa = a;
		if (modes[1]!=1)
			bb = list[b] || 0;
		else
			bb = b;
		if (modes[2]!=1)
			cc = list[c] || 0;
		else
			cc = c;

		if (debug)
			console.log(list.slice(index,index+4));
		if (debug)
			console.dir({block, op, modes, a ,aa,b,bb,c,cc});
		if (op == 1) {
			if (debug)
				console.log(`$1 [${c}]<=${aa}+${bb}`);
			list[c]=aa+bb;
			index += 4;
		} else if (op == 2) {
			if (debug)
				console.log(`$2 [${c}]<=${aa}*${bb}`);
			list[c]=aa*bb;
			index += 4;
		} else if (op == 3) {
			x = suppliedInputs.pop();
			if (debug)
				console.log(`$3 [${a}]<=${x}`);
			list[a] = x;
			index += 2;
		} else if (op == 4) {
			if (debug)
				console.log(`$4 [${a}]=> ${aa}`);
			output.push(list[a]);
			index += 2;
		} else if (op == 5){
			if (debug)
				console.log(`$5 JMP ${aa}==true ? GOTO ${bb}`);
			if (aa!=0) {
				index = bb;
			} else {
				index += 3;
			}
		} else if (op == 6) {
			if (debug)
				console.log(`$6 JMP ${aa}==false ? GOTO ${bb}`);
			if (aa==0) {
				index = bb;
			} else {
				index += 3;
			}
		} else if (op == 7) {
			if (debug)
				console.log(`$7 [${c}] <= ${aa} LT ${bb}`);
			list[c] = aa < bb ? 1 : 0;
			index+=4;
		} else if (op == 8) {
			if (debug)
				console.log(`$8 [${c}] <= ${aa} EQ ${bb}`);
			list[c] = aa == bb ? 1 : 0;
			index+=4;
		} else if (op == 99) {
			if (debug)
				console.log(`$99 BREAK`);
			return output;
		} else {
			throw `Bad opcode <${op}> from block [${block}] at position ${index}`;
		}
	}
}

function permute(input) {
	const rtn = [];
	const used = [];
	function step(input) {
		let i, char;
		for (i=0; i<input.length; i++) {
			char = input.splice(i, 1)[0];
			used.push(char);
			if (input.length == 0)
				rtn.push(used.slice());
			step(input);
			input.splice(i, 0, char);
			used.pop();
		}
	}
	step(input);
	return rtn;
}

let max = {in: [], out: -1};
main:
for (const set of permute([0,1,2,3,4])) {
	console.log(`Attempting ${set}`);
	let prev = 0;
	for (const x of set) {
		try {
			const y = [prev, x];
			console.log(`Passing in ${y}`)
			prev = parseInt(intcode(input, y));
			console.log(`Got ${prev}`);
		} catch (e) {
			console.log(`Failed on ${x}`);
			//continue main;
		}
	}
	if (prev > max.out) {
		console.log(`${prev} > ${max.out}, ${set} is new max ${JSON.stringify(max)}`);
		max.out = prev;
		max.in = set;
	}
}
console.log("Found maximum thruster configuration:");
console.dir(max);
