type YEARS = 2020 | 2021 | 2022;

export async function loadInput(year: YEARS, day: number): Promise<string> {
    console.log('session', Deno.env.get('AOC_SESSION')!);
    
    try {
        await Deno.stat('input.txt');
        return await Deno.readTextFile('input.txt');
    } catch {
        const res = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
            headers: {
                cookie: `session=${Deno.env.get('AOC_SESSION')!}`,
                'user-agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36'
            },
        });
        const input = await res.text();
        await Deno.writeTextFile('input.txt', input);
        return input;
    }
}
