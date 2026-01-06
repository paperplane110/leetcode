function rotate(matrix: number[][]): void {
  const n = matrix.length;
  for (let row = 0; row < n; row++) {
    for (let col = 0; col < row; col++) {
      [matrix[row][col], matrix[col][row]] = [matrix[col][row], matrix[row][col]]
    }
  }
  for (const row of matrix) {
    row.reverse()
  }
}