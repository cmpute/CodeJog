TARGET = 1000

def solve(target = TARGET):
    for i in range(target):
        for j in range(3, i):
            k = target - i - j
            if k < i : break
            if i * i + j * j == k * k:
                return i * j * k

if __name__ == "__main__":
	print(solve())
