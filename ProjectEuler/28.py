TARGET = 1001

def solve(target = TARGET):
    current = 1
    diff = 2
    dsum = 1
    while diff < target:
        # c, c+d, c+2d, c+3d, c+4d
        dsum += 4 * current + 10 * diff
        current += 4 * diff
        diff += 2
    return dsum

if __name__ == "__main__":
	print(solve())
