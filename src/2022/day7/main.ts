enum NodeType {
    File,
    Dir,
}

interface Node {
    type: NodeType;
    name: string;
    parent: Node | null;

    get size(): number;
}

class File implements Node {
    type = NodeType.File;
    name: string;
    parent: Node | null;
    #size = 0;

    constructor(name: string, parent: Dir, size: number) {
        this.name = name;
        this.size = size;
        this.parent = parent;
    }

    set size(size: number) {
        this.#size = size;
    }

    get size() {
        return this.#size;
    }
}

class Dir implements Node {
    type = NodeType.Dir;
    name: string;
    parent: Dir | null;

    onGetSize?: (node: Node) => number;

    constructor(name: string, parent: Dir | null, onGetSize: (node: Node) => number) {
        this.name = name;
        this.parent = parent;
        this.onGetSize = onGetSize;
    }

    get size(): number {
        return this.onGetSize ? this.onGetSize(this) : 0;
    }
}

class System {
    totalSize = 70_000_000;

    fs: Node[] = [];
    root: Dir;
    cwd: Dir;

    constructor() {
        this.root = new Dir('/', null, this.getDirSize);
        this.cwd = this.root;
        this.fs.push(this.root);
        this.getDirSize = this.getDirSize.bind(this);
    }

    getDirSize = (node: Node): number => {
        return this.fs.reduce((acc, file) => {
            return file.parent === node ? acc + file.size : acc;
        }, 0);
    };

    createFile(name: string, size: number): Node {
        const newFile = new File(name, this.cwd, size);
        this.fs.push(newFile);
        return newFile;
    }

    createDir(name: string): Node {
        const newDir = new Dir(name, this.cwd, this.getDirSize);
        this.fs.push(newDir);
        return newDir;
    }

    findDir(name: string): Node | undefined {
        if (name === '/') {
            return this.root;
        }
        return this.fs.find((file) => file.parent === this.cwd && file.name === name);
    }

    changeDir(name: string) {
        if (name === '..') {
            this.cwd = this.cwd.parent ?? this.cwd;
        } else {
            this.cwd = this.findDir(name) ?? this.createDir(name);
        }
        return this.cwd;
    }

    getUsedSpace(): number {
        return this.root.size;
    }

    getFreeSpace(): number {
        return this.totalSize - this.getUsedSpace();
    }

    parseHistory(history: string[]) {
        for (const line of history) {
            switch (true) {
                case line.startsWith('$ cd'):
                    this.changeDir(line.split(' ')[2]);
                    break;
                case line.startsWith('dir '):
                    this.createDir(line.split(' ')[1]);
                    break;
                case line.startsWith('$ ls'): // nop
                    break;
                default: {
                    const [size, name] = line.split(' ');
                    this.createFile(name, +size);
                }
            }
        }
    }
}

try {
    const input = await Deno.readTextFile('./input.txt');

    const sys = new System();
    sys.parseHistory(input.split('\n').map(i => i.trim()));

    // Part I: 1141028
    const part1 = sys.fs.reduce((acc, node) =>
        (node.type === NodeType.Dir && node.size <= 100_000) ? acc + node.size : acc, 0); //?

    console.log('Part I: ', part1);

    // Part II: 8278005
    const neededSize = 30_000_000;
    const toFreeSize = neededSize - sys.getFreeSpace();
    const bestCandidates = sys.fs.filter(n => n.type === NodeType.Dir && n.size >= toFreeSize)
        .map(n => n.size)
        .sort((a, b) => a - b);

    console.log('Part II:', bestCandidates[0]);

} catch (e) {
    // console.error('Input file not found');
    Deno.exit(1);
}
