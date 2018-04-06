EXP = 1000

def solve(exp = EXP):
    '''
    Brute-force solution
    '''
    return sum(int(d) for d in str(2**exp))

if __name__ == "__main__":
	print(solve())
