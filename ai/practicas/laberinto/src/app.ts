import P5 from "p5";
import "p5/lib/addons/p5.dom";
import "./styles.scss";

const cols = 50;
const rows = 50;
const grid = new Array(cols)
const width = 500;
const height = 500;

let Status = {
    NONE: [255, 255, 255],
    START: [0, 255, 0],
    END: [255, 0, 0],
    WALL: [0, 0, 0],
    VISITED: [255, 255, 0],
    PATH: [0, 255, 255]
}

// Creating the sketch itself
const sketch = (p5: P5) => {
    // The sketch setup method
    p5.setup = () => {
        // Creating and positioning the canvas
        const canvas = p5.createCanvas(width, height);
        canvas.parent("app");

        // Configuring the canvas
        p5.background("white");

        // Creating the grid
        for (let i = 0; i < cols; i++) {
            grid[i] = new Array(rows);
        }

        for (let i = 0; i < cols; i++) {
            for (let j = 0; j < rows; j++) {
                grid[i][j] = {
                    status: Math.random() < 0.2 ? Status.WALL : Status.NONE,
                    x: i,
                    y: j,
                }
            }
        }
        grid[0][0].status = Status.START;
        grid[cols - 1][rows - 1].status = Status.END;
    };

    // The sketch draw method
    p5.draw = () => {
        let w = width / cols;
        let h = height / rows;
        // Drawing the grid
        for (let i = 0; i < cols; i++) {
            for (let j = 0; j < rows; j++) {
                p5.stroke(0);
                p5.fill(grid[i][j].status);
                p5.rect(i * w, j * h, w - 1, h - 1);
            }
        }
    };
};

new P5(sketch);
