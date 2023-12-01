// var content = `[({(<(())[]>[[{[]{<()<>>
// [(()[<>])]({[<{<<[]>>(
// {([(<{}[<>[]}>{[]{[(<()>
// (((({<>}<{<{<>}{[]{[]{}
// [[<[([]))<([[{}[[()]]]
// [{[{({}]{}}([{[{{{}}([]
// {<[[]]>}<{[{[{[]{()[[[]
// [<(<(<(<{}))><([]([]()
// <{([([[(<>()){}]>(<<{{
// <{([{{}}[<[[[<>{}]]]>[]]`;

// var content = `{([(<{}[<>[]}>{[]{[(<()>
// [[<[([]))<([[{}[[()]]]
// [{[{({}]{}}([{[{{{}}([]
// [<(<(<(<{}))><([]([]()
// <{([([[(<>()){}]>(<<{{`;

var content = document.querySelector("pre").innerText;
var input = content
	.trim()
	.split("\n");

var wrongScores = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
};

var autocompleteScores = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4,
};

var open2close = {
    "(":")",
    "[":"]",
    "{":"}",
    "<":">",
    ")":")",
    "]":"]",
    "}":"}",
    ">":">",
};

var close2open = {
    ")":"(",
    "]":"[",
    "}":"{",
    ">":"<",
    "(":"(",
    "[":"[",
    "{":"{",
    "<":"<",
};

var changes = {
    ")":-1,
    "]":-1,
    "}":-1,
    ">":-1,
    "(":1,
    "[":1,
    "{":1,
    "<":1,
}

function getAutocompleteScore(closeStack) {
    var total = 0;
    while (closeStack.length > 0) {
        total *= 5;
        total += autocompleteScores[closeStack.pop()];
    }
    return total;
}

function getScore(line) {
    var stack = {
        ")": 0,
        "]": 0,
        "}": 0,
        ">": 0,
    };
    var closeStack = [];
    for (const char of line) {
        var change = changes[char];
        stack[open2close[char]]+=change;
        // console.log(char, id, change, nextClosing);
        if (change > 0) {
            closeStack.push(open2close[char]);
        } else {
            var shouldClose = closeStack.pop();
            if (char !== shouldClose) {
                return {
                    expected: shouldClose,
                    got: char,
                    error: "wrong",
                    score: wrongScores[char],
                };
            }
            if (stack[char]<0) {
                return {
                    expected: close2open[char],
                    got: char,
                    error: "unopened",
                    score: wrongScores[char],
                };
            }
        }
    }
    return {
        error: "incomplete",
        score:getAutocompleteScore(closeStack)
    };
}
function sumScores(x) {
    return x.reduce((a,b) => a+b.score,0);
}
var lineScores = input.map(line => getScore(line));
var corruptLines = lineScores.filter(x => x.error==="wrong");
console.log(lineScores, corruptLines);
console.log("corrupt(part1)", sumScores(corruptLines));

var incompleteScores = lineScores.filter(x => x.error==="incomplete");
incompleteScores.sort((a,b) => b.score-a.score);
console.log("mid(part2)", incompleteScores[Math.floor(incompleteScores.length/2)].score);