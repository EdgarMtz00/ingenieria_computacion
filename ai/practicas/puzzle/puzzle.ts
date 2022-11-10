enum Movement {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3
}

export type Puzzle = {
    board: number[][],
    blank_row: number,
    blank_col: number,
    previous?: Movement
}

export type TreeNode = {
    value: Puzzle,
    leafs: TreeNode[]
}

function new_puzzle(p: Puzzle): Puzzle {
    return {
        board: p.board.map(row => row.slice()),
        blank_row: p.blank_row,
        blank_col: p.blank_col
    }
}

function move_puzzle(puzzle: Puzzle, move: number): Puzzle | null {
    let newPuzzle = new_puzzle(puzzle);
    switch (move) {
        case Movement.Up:
            if (puzzle.blank_row === 0 || puzzle.previous === Movement.Down) {
                return null;
            }
            newPuzzle.blank_row -= 1;
            newPuzzle.previous = Movement.Up;
            break;
        case Movement.Down:
            if (puzzle.blank_row === puzzle.board.length - 1 || puzzle.previous === Movement.Up) {
                return null;
            }
            newPuzzle.blank_row += 1;
            newPuzzle.previous = Movement.Down;
            break;
        case Movement.Left:
            if (puzzle.blank_col === 0 || puzzle.previous === Movement.Right) {
                return null;
            }
            newPuzzle.blank_col -= 1;
            newPuzzle.previous = Movement.Left;
            break;
        case Movement.Right:
            if (puzzle.blank_col === puzzle.board.length - 1 || puzzle.previous === Movement.Left) {
                return null;
            }
            newPuzzle.blank_col += 1;
            newPuzzle.previous = Movement.Right;
            break;
    }

    let temp = newPuzzle.board[newPuzzle.blank_row][newPuzzle.blank_col];
    newPuzzle.board[newPuzzle.blank_row][newPuzzle.blank_col] = newPuzzle.board[puzzle.blank_row][puzzle.blank_col];
    newPuzzle.board[puzzle.blank_row][puzzle.blank_col] = temp;
    return newPuzzle;
}

export function match_puzzles(puzzle: Puzzle, goal: Puzzle): boolean {
    for (let row = 0; row < puzzle.board.length; row++) {
        for (let col = 0; col < puzzle.board[row].length; col++) {
            if (puzzle.board[row][col] !== goal.board[row][col]) {
                return false;
            }
        }
    }
    return true;
}

export function print_board(puzzle: Puzzle) {
    for (let row = 0; row < puzzle.board.length; row++) {
        console.log(puzzle.board[row].join(" "));
    }
    console.log();
}

const puzzleMemo: Map<string, Puzzle> = new Map();
export function expand_puzzle(puzzle: Puzzle): Puzzle[] {
    let leafs: Puzzle[] = [];

    for(let value = 0; value < 4; value++) {
        let newPuzzle = move_puzzle(puzzle, value);
        if (newPuzzle !== null && !puzzleMemo.has(newPuzzle.board.toString())) {
            puzzleMemo.set(newPuzzle.board.toString(), newPuzzle);
            leafs.push(newPuzzle);
        }
    }
    return leafs;
}

