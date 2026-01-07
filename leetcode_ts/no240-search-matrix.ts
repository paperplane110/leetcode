function searchMatrix(matrix: number[][], target: number): boolean {
    const m = matrix.length, n = matrix[0].length;
    let row = 0, col = n - 1;
    while (row < m && col >= 0) {
        const rt = matrix[row][col]
        if (rt > target) {
            col--;
        } else if (rt < target) {
            row++;
        } else {
            return true;
        }
    }
    return false;
}