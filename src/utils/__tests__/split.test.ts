import { assertEquals } from 'https://deno.land/std@0.167.0/testing/asserts.ts';
import { splitArray } from '../split.ts';


Deno.test('split array by same type element', () => {
    const input = [1, 0, 3, 4, 5, 0, 6, 7];
    const result = splitArray(input, 0);
    const output = [[1], [3, 4, 5], [6, 7]];

    assertEquals(result, output);
});
