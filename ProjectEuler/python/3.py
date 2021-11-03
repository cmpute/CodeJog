TARGET = 600851475143

def solve(target = TARGET):
    from em import factors
    return max(factors(target))

if __name__ == "__main__":
	print(solve())
