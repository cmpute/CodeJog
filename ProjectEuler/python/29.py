LIMIT = 100

def solve(limit = LIMIT):
    '''
    Brute-force solution
    '''
    result = set()
    for a in range(2, limit + 1):
        for b in range(2, limit + 1):
            result.add(a**b)
    return len(result)

if __name__ == "__main__":
	print(solve())
