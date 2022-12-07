import { loadInput } from "../../utils/load-input.ts";

/**
 * For each value in the input, add it to the uniqSeq array,
 * and if it's already in the array, remove all values before it.
 *
 * @param input - text string
 * @param size - the size of the sequence we're looking for
 * @returns The index of the first unique sequence of size n.
 */
function findFirstUniqSeq(input: string, size: number): number | undefined {
    const uniqSeq: string[] = [];
    for (let i = 0; i < input.length - size; i++) {
        const val = input[i];
        uniqSeq.push(val);
        for (let j = uniqSeq.length - 2; j >= 0; j--) {
            if (uniqSeq[j] === val) {
                for (let k = j; k >= 0; k--) {
                    uniqSeq.shift(); // ?
                }
                break;
            }
        }
        if (uniqSeq.length === size) {
            return i + 1; // uniq full set! return next index
        }
    }
}

const input = await loadInput(2022, 6);

// Part 1: cnt=4; result: 1816
// Part 2: cnt=14; result: 2625
console.log([4, 14].map((cnt) => findFirstUniqSeq(input, cnt)));
