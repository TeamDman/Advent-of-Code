const min = 359282;
const max = 820401;
let total = 0;
main:
for (let i=min; i<=max; i++) {
    if ((i+"").match(/(\d)\1/) === null)
        continue;
    let high = (i+"")[0]+0;
    for (const c of i+"") {
        if (high > c+0) {
            continue main;
        } else {
            high=c+0;
        }
    }
    total++;
}
console.log(total);