type SeparatorFn<T> = (value: T, index: number, array: T[]) => boolean;
type ArraySeparator<T> = T | SeparatorFn<T>;

export function splitArray<T>(array: T[], separator: ArraySeparator<T>): T[][] {
    const result: T[][] = [];
    let current: T[] = [];
    for (const idx in array) {
        const value = array[+idx];
        const isEqual = separator === value;
        const isSeparatorFn = typeof separator === 'function';
        // @ts-ignore - TS doesn't know that the separator is a function
        if (isEqual || (isSeparatorFn && separator(value, +idx, array))) {
            result.push(current);
            current = [];
        } else {
            current.push(value);
        }
    }
    result.push(current);
    return result;
}

splitArray([1, 0, 3, 4, 5, 0, 6, 7], 0); // ?
