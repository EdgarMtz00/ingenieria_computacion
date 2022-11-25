type Nodes = {
    x: number,
    y: number,
}

function NewNode(x: number, y: number): Nodes {
    return { x, y }
}

class BreadthSearch {
    private tree: Nodes;
    private queue: Nodes[];

    constructor() {
        this.tree = null;
        this.queue = [NewNode(0, 0)];
    }

    public search_step(grid: any) {

    }
}