def solve():
    '''
    Brute-force solution

    Note
    ----
    100*100=10000 already contains at least 11 digits,
    so either multiplier or multiplicand have to be below 100
    99*99=9801 contains 8 digits so one of above also have to be above 100
    '''
    candidates = []
    for i in range(100):
        for j in range(100, 10000):
            combine = str(i) + str(j) + str(i*j)
            if(len(combine) != 9):
                continue
            if(''.join(sorted(combine)) == '123456789'):
                candidates.append((i, j))

    # return sum(a*b for a,b in candidates)
    # use set to remove duplicate products
    return sum(set(a*b for a,b in candidates))

if __name__ == "__main__":
	print(solve())
