INDEX = 1000000

def solve(index = INDEX):
    '''
    Brute-force solution
    '''
    from itertools import permutations
    combination = permutations(range(10))
    while index > 0:
        index -= 1
        result = next(combination)
    return ''.join(str(d) for d in result)

if __name__ == "__main__":
	print(solve())
