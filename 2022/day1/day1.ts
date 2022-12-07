import { loadInput } from '../../utils/load-input.ts';

const input = await loadInput(2022, 1);

/**
 * It takes a string, splits it into lines, converts each line into a number, and then sums the numbers
 * in each group of lines
 * @param {string} input - string - the input string
 * @returns top theree sums
 */
function getMaxSum(input: string): number[] {
    const lines: Array<number | undefined> = input.split('\n').map((line) => (line.length === 0 ? undefined : parseInt(line, 10)));

    const allSum: number[] = [];
    let currentSum = 0;
    lines.forEach((value) => {
        if (value === undefined) {
            allSum.push(currentSum);
            currentSum = 0;
        } else {
            currentSum += value;
        }
    });

    return allSum.sort((a, b) => b - a).slice(0, 3);
}

const leaders = getMaxSum(input);

console.log('top three:', leaders);
console.log(
    'top three sum:',
    leaders.reduce((acc, prev) => acc + prev),
);
