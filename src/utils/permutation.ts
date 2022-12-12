export const mul = a => a.reduce((prev, item) => prev * item, 1);
export const sum = a => a.reduce((prev, item) => prev + item, 0);

export function permutation<T>(a: T[], n: number, cb = (v: T[]): any => v): T[][] {
    const result = [];
    for (let i = 0; i <= a.length - n; i++) {
        for (let j = i + 1; j <= a.length - n + 1; j++) {
            result.push(cb([a[i], ...a.slice(j, j + n - 1)], i, a, j));
        }
    }
    return result;
}
