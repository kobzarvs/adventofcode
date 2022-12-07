type YEARS = 2020 | 2021 | 2022;

export async function loadInput(year: YEARS, day: number): Promise<string> {
    const url = `https://adventofcode.com/${year}/day/${day}/input`;
    const session = 'session=' + Deno.env.get('AOC_SESSION')!;
    console.log(session);
    console.log('url:', url);

    try {
        await Deno.stat('input.txt');
        return await Deno.readTextFile('input.txt');
    } catch {
        const res = await fetch(url, {
            headers: {
                cookie: session,
                'user-agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36',
            },
        });
        const input = await res.text();
        try {
            await Deno.writeTextFile('input.txt', input);
        } catch (e) {
            console.log('Error writing input.txt', e);
        }
        return input;
    }
}

export async function loadTextFile(filename: string): Promise<string | undefined> {
    try {
        await Deno.stat(filename);
        return await Deno.readTextFile(filename);
    } catch (e) {
        console.log('Error writing input.txt', e);
    }
}
