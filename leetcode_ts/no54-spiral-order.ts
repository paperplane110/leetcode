function spiral_order(matrix: number[][]): number[] {
    const rowCnt = matrix.length;
    const colCnt = matrix[0].length;
    const totalNum = rowCnt * colCnt
    const ans: number[] = [];
    let lt = 1, rt = 0, rb = 0, lb = 0;
    let row = 0, col = 0
    let currDir = "right"
    while (ans.length < totalNum) {
        if (currDir === "right") {
            while (col < colCnt - rt) {
                ans.push(matrix[row][col]);
                col += 1;
            }
            row++;
            col--;
            rt++;
            currDir = "down"

        } else if (currDir === "down") {
            while (row < rowCnt - rb) {
                ans.push(matrix[row][col]);
                row++;
            }
            row--;
            col--;
            rb++;
            currDir = "left";

        } else if (currDir === "left") {
            while (col >= lb) {
                ans.push(matrix[row][col]);
                col--;
            }
            row--;
            col++;
            lb++;
            currDir = "up";
        } else {
            while (row >= lt) {
                ans.push(matrix[row][col]);
                row--;
            }
            col++;
            row++;
            lt++;
            currDir = "right";
        }
    }
    return ans
}

(function main() {
    const test = [[1,2,3],[4,5,6],[7,8,9]];
    console.log(spiral_order(test))
})()