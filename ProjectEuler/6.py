LIMIT = 100

def solve(limit = LIMIT):
    '''
    Note: (1+2+...+n)^2=1^3+2^3+...n^3
    '''
    return sum((n-1) * n * n for n in range(limit + 1))

if __name__ == "__main__":
	print(solve())
