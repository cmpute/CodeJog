LIMIT = 2000000

def solve(limit = LIMIT):
    from em import primes
    return sum(primes(limit))

if __name__ == "__main__":
	print(solve())
