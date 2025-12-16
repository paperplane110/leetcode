from typing import List
import copy


def exist(board: List[List[str]], word: str) -> bool:

    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    visited = set()

    def search_letter(word, row, col):
        if board[row][col] != word[0]:
            return False

        visited.add((row, col))
        if len(word) == 1:
            return True
        for direction in directions:
            new_row = row + direction[0]
            new_col = col + direction[1]
            if new_row >= len(board) or new_row < 0 or new_col >= len(board[0]) or new_col < 0:
                # out of the board
                continue
            if (new_row, new_col) in visited:
                continue
            if search_letter(word[1:], new_row, new_col):
                return True
        visited.remove((row, col))
        return False

    for row in range(len(board)):
        for col in range(len(board[0])):
            if search_letter(word, row, col):
                return True
    return False


# board = [["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]]
# word = "ABCCED"
# word = "SEE"

# board = [["a"]]
# word = "a"

# board = [["a", "a", "a", "a"], ["a", "a", "a", "a"], ["a", "a", "a", "a"]]
# word = "aaaaaaaaaaaaa"

# board = [["C", "A", "A"], ["A", "A", "A"], ["B", "C", "D"]]
# word = "AAB"

board = [["A", "B", "C", "E"], ["S", "F", "E", "S"], ["A", "D", "E", "E"]]
word = "ABCESEEEFS"
print(exist(board, word))
