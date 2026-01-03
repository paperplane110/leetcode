function setZero(matrix: number[][]): void {
  const rowNum = matrix.length;
  const colNum = matrix[0].length;
  const zeroRow: Set<number> = new Set();
  const zeroCol: Set<number> = new Set();
  for (let row = 0; row < rowNum; row++) {
    for (let col = 0; col < colNum; col++) {
      if (matrix[row][col] === 0) {
        zeroRow.add(row);
        zeroCol.add(col);
      }
    }
  }
  for (let row = 0; row < rowNum; row++) {
    for (let col = 0; col < colNum; col++) {
      if (zeroRow.has(row) || zeroCol.has(col)) {
        matrix[row][col] = 0;
      }
    }
  }
}