const x = null;

const schedule = [
    19,
    x,x,x,x,x,x,x,x,41,
    x,x,x,37,
    x,x,x,x,x,367,
    x,x,x,x,x,x,x,x,x,x,x,x,13,
    x,x,x,17,
    x,x,x,x,x,x,x,x,x,x,x,29,
    x,373,
    x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23
];


// 67*7*59*61 - 67*4002 - 7*9397 - 59*5000 - 61*5000
// 67*7*59*61 - (67*4812) - (7*4824) - (59*4814) - (61*4815)
// 67*7*59*61 - (67*4814) - (7*4814) - (59*4814) - (61*4814)
// (n) => 67*7*59*61 - (67*n+4*n) - (7*n+n*3) - (59*n+n*2) - (61*n+n) -1

function run(schedule = [67, 7, 59, 61]) {
    const x = null;

    const MAX_TS = 1_000_000_000;

    const MASK = '1'.repeat(schedule.length);
    const step = schedule[0] || 1;

    const matchOccurs = (ts) => {
        return schedule.map((n, i) =>
            !n || (ts+i) % n === 0 ? '1' : '0'
        ).join('')
    }

    let count = 0;
    let prevTs = 0;
    let countTs = 0;

    for(let ts = 0; ts <= MAX_TS + schedule.length + 1; ts+=step, count++) {
        const mask = matchOccurs(ts);
        if (mask === MASK) {
            countTs++;
            console.log('ts:', ts, 'delta:', ts-prevTs, 'count:', count, JSON.stringify(schedule));
            prevTs = ts;
            if (countTs ===5 || count > 1_000_000) {
                break;
            }
        }
    }
    console.log('ts not found in first', count, 'times');
}
