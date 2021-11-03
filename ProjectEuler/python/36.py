LIMIT = 1000000

def solve(limit = LIMIT):
    '''
    Brute-force solution
    '''
    return sum(i for i in range(limit) 
               if str(i) == str(i)[::-1]
               and bin(i)[2::] == bin(i)[:1:-1])

if __name__ == "__main__":
	print(solve())
