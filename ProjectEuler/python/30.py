EXP = 5

def solve(exp = EXP):
    '''
    Brute-force solution
    '''
    powdict = dict()
    maxlen = 1
    total = 0
    for i in range(10):
        powdict[str(i)] = i ** exp
    while int(''.join(['1'] + (maxlen - 1)*['9'])) < maxlen * powdict['9']:
        maxlen += 1
    for i in range(2, maxlen * powdict['9']):
        if sum(powdict[d] for d in str(i)) == i:
            total += i
    return total

if __name__ == "__main__":
	print(solve())
