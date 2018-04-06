SIZE = 20

def solve(size = SIZE):
    '''
    Solution is C(size, 2*size)
    '''
    total = 1
    for i in range(size+1, 2*size+1):
        total *= i
    for i in range(1,size+1):
        total //= i
    return total

if __name__ == "__main__":
	print(solve())
