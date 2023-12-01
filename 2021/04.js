input = document.querySelector("pre").innerText
	.trim()
	.split("\n");

draws = input[0].split(",").map(x => parseInt(x));
boards = input
	.slice(1)
	.join("\n")
	.split("\n\n")
	.map(board => board.trim().split("\n").map(line => line.trim().split(/\s+/).map(item => parseInt(item))));

drawn = [];

function isWinner(board, drawn) {
	for (i=0; i<board.length; i++) {
		if ([...Array(board.length).keys()].map(x => board[x][i]).every(x => drawn.includes(x))) return true;
		if (board[i].every(x => drawn.includes(x))) return true;
	}
	return false;
}

function sum(board, drawn) {
	x = 0;
	for (i=0; i<board.length; i++) {
		for (j=0; j<board[0].length; j++) {
			z = board[i][j];
			if (!drawn.includes(z)) {
				x += z;
			}
		}
	}
	return x;
}

_draws = draws.slice()
while (true) {
	drawn.push(_draws.shift());
	winner = boards.filter(b => isWinner(b, drawn));
	if (winner.length > 0) {
		a = sum(winner[0], drawn);
		console.log("result="+a*drawn.pop());
		break;
	}
}

drawn = [];
_draws = draws.slice()
while (true) {
	drawn.push(_draws.shift());
	winner = boards.filter(b => isWinner(b, drawn));
	if (winner.length === boards.length) {
		worst = boards.filter(b => !isWinner(b, drawn.slice(0, drawn.length-1)))[0];
		console.log(worst, drawn);
		a = sum(worst, drawn);
		console.log("result="+a*drawn.pop());
		break;
	}
}