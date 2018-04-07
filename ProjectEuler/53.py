MINIMUM = 1000000
LIMIT = 100

def solve(minimum = MINIMUM, limit = LIMIT):
    '''
    Using YangHui's Triangle (Pascal's Triangle)
    '''
    total = 0
    last_line = [1, 5, 10, 10, 5, 1] # start from 6
    while len(last_line) <= limit:
        line = [1]
        for i in range(len(last_line) - 1):
            line.append(last_line[i] + last_line[i+1])
        line.append(1)
        total += sum(num > minimum for num in line)
        last_line = line
    return total

if __name__ == "__main__":
	print(solve())
