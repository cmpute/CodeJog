LIMIT = 20

def solve(limit = LIMIT):
    from em import gcd
    
    total = 1
    for i in range(2, limit + 1):
        total = total * i // gcd(total, i)
    return total

if __name__ == "__main__":
	print(solve())
