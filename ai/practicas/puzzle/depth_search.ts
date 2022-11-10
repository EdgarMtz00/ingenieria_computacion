import {expand_puzzle, match_puzzles, print_board, Puzzle, TreeNode} from './puzzle.ts';

export function depth_search(puzzle: Puzzle, goal: Puzzle): TreeNode | null {
    const root: TreeNode = {
        value: puzzle,
        leafs: []
    }
    const stack: TreeNode[] = [root];
    while (stack.length > 0) {
        const node = stack.pop();
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
            stack.push(child);
        }
    }
    return null;
}

const puzzle: Puzzle = {
    board: [[0, 5, 2], [1, 4, 3], [7, 8, 6]],
    blank_row: 0,
    blank_col: 0
}

const goal: Puzzle = {
    board: [[1, 2, 3], [4, 5, 6], [7, 8, 0]],
    blank_row: 2,
    blank_col: 2
}

depth_search(puzzle, goal);