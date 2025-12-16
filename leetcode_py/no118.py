def generate(num_rows: int) -> list[list[int]]:
    ans = []
    for row in range(num_rows):
        if row == 0:
            ans.append([1])
        elif row == 1:
            ans.append([1, 1])
        else:
            tmp = []
            for i in range(row+1):
                if i == 0 or i == row:
                    tmp.append(1)
                else:
                    tmp.append(ans[row-1][i-1] + ans[row-1][i])
            ans.append(tmp.copy())
    return ans

print(generate(4))