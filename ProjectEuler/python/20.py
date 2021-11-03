TARGET = 100

def solve(target = TARGET):
    '''
    Brute-force solution using python built-in
    '''
    from math import factorial
    return sum(int(c) for c in str(factorial(100)))

if __name__ == "__main__":
	print(solve())
