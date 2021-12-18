//https://stackoverflow.com/a/42919752/11141271
{
    const top=0,parent=c=>(c+1>>>1)-1,left=c=>(c<<1)+1,right=c=>c+1<<1;class PriorityQueue{constructor(c=(d,e)=>d>e){this._heap=[],this._comparator=c}size(){return this._heap.length}isEmpty(){return 0==this.size()}peek(){return this._heap[top]}push(...c){return c.forEach(d=>{this._heap.push(d),this._siftUp()}),this.size()}pop(){const c=this.peek(),d=this.size()-1;return d>top&&this._swap(top,d),this._heap.pop(),this._siftDown(),c}replace(c){const d=this.peek();return this._heap[top]=c,this._siftDown(),d}_greater(c,d){return this._comparator(this._heap[c],this._heap[d])}_swap(c,d){[this._heap[c],this._heap[d]]=[this._heap[d],this._heap[c]]}_siftUp(){for(let c=this.size()-1;c>top&&this._greater(c,parent(c));)this._swap(c,parent(c)),c=parent(c)}_siftDown(){for(let d,c=top;left(c)<this.size()&&this._greater(left(c),c)||right(c)<this.size()&&this._greater(right(c),c);)d=right(c)<this.size()&&this._greater(right(c),left(c))?right(c):left(c),this._swap(c,d),c=d}}window.PriorityQueue=PriorityQueue
}

var content = document.querySelector("pre").innerText;

var content=`1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`;

var input = content
    .trim()
    .split("\n")
    .map(row => row.split("").map(x => parseInt(x)));



var dest = [input.length-1, input[0].length-1];

var scoreToBeat=0;
var leftDownTotal = 0;
var i, j;
for (i=1; i<input.length; i++) leftDownTotal += input[i][0];
for (j=1; j<input[0].length; j++) leftDownTotal += input[i-1][j];
var rightAcrossTotal = 0;
for (j=1; j<input[0].length; j++) rightAcrossTotal += input[0][j];
for (i=1; i<input.length; i++) rightAcrossTotal += input[i][j-1];
var diagTotal = 0;
i=0; j=0;
while (true) {
    if (i<input.length-1) i++;
    diagTotal += input[i][j];
    if (j<input[0].length-1) j++;
    diagTotal += input[i][j];
    if (i >= input.length-1 && j >= input[0].length-1) break;
}
scoreToBeat = Math.min(leftDownTotal, rightAcrossTotal, diagTotal);

var paths = new PriorityQueue((a,b) => a.totalCost < b.totalCost);
paths.push({
    points: [[0,0]],
    totalCost: 0,//input[0][0]
});

function getAdj(i,j) {
    var above = [i-1, j];
    var below = [i+1, j];
    var left = [i, j-1];
    var right = [i, j+1];
    var all = [above, below, left, right]
        .map(([i,j]) => [input[i]?.[j], [i,j]])
        .filter(x => x[0]!==undefined);
    return all;
}

function eq(a,b) {
    return a[0]==b[0] && a[1]==b[1]
}

function getOptions(path) {
    var points = path.points;
    var point = points[points.length-1];
    return getAdj(...point).filter(x => !points.some(p => eq(p,x[1])));
}

// function prune(paths) {
//     return Object.values(paths.reduce((map, cur) => {
//         if (cur.totalCost > scoreToBeat) return map;
//         var last = cur.points[cur.points.length-1];
//         var key = last[0] + " " + last[1];
//         var existing = map[key];
//         if (existing === undefined || existing.totalCost > cur.totalCost) {
//             map[key] = cur;
//         }
//         return map;
//     }, {}));
// }

var distCosts = {};

function step() {
    // paths=prune(paths);
    path = paths.pop();
    var newPaths = getOptions(path)
        .map(point => ({
            points: [...path.points, point[1]],
            totalCost: path.totalCost + point[0],
        }));
    for (const next of newPaths) {
        var latest = next.points[next.points.length-1];
        if (eq(latest, dest)) {
            return next;
        }
        var key = latest[0] + " " + latest[1];
        if (distCosts[key]?.totalCost < next.totalCost) continue;
        distCosts[key] = next;
        paths.push(next);
    }
}

function sleep(time) {
    return new Promise((res, rej) => setTimeout(res, time));
}

var go=true;
async function main() {
    for (var i=0;go; i++) {
        if (i%10000==0) {
            console.log("Step",i,paths.size());
            await sleep(1);
        }
        var win = step();
        // var win = paths.find(p =>  eq(p.points[p.points.length-1], dest));
        if (win !== undefined) break;
    }
    if (win !== undefined)
        console.log(win.totalCost);
}
await main();
