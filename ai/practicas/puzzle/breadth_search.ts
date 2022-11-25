import {match_puzzles, Puzzle, print_board, expand_puzzle, TreeNode} from './puzzle.ts';

export function breadth_search(puzzle: Puzzle, goal: Puzzle): TreeNode | null {
    const root: TreeNode = {
        value: puzzle,
        leafs: []
    }
    const queue: TreeNode[] = [root];
    while (queue.length > 0) {
        const node = queue.shift();
        if (node === undefined) {
            return null;
        }
        print_board(node.value);
        if (match_puzzles(node.value, goal)) {
            return node;
        }
        const leafs = expand_puzzle(node.value);
        for (const leaf of leafs) {
            const child: TreeNode = {
                value: leaf,
                leafs: []
            }
            node.leafs.push(child);
            queue.push(child);
        }
    }
    return null;
}

const puzzle: Puzzle = {
    board: [[1, 2, 3], [4, 5, 6], [0, 7, 8]],
    blank_row: 0,
    blank_col: 0
}

const goal: Puzzle = {
    board: [[1, 2, 3], [4, 5, 6], [7, 8, 0]],
    blank_row: 2,
    blank_col: 2
}

breadth_search(puzzle, goal);