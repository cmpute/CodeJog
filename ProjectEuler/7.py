TARGET = 10001

def solve(target = TARGET):
    from em import nprimes
    return nprimes(target)[target - 1]

if __name__ == "__main__":
	print(solve())
