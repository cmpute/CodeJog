LIMIT = 1000

def solve(limit = LIMIT):
    from em import powmod
    mask = 10 ** 10
    total = sum(powmod(i, i, mask) for i in range(1, limit + 1))
    return total % mask

if __name__ == "__main__":
	print(solve())
