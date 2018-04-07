FMIN = 500

def solve(fmin = FMIN):
    '''
    Note: Alternative solution: Pollard's rho algorithm
    '''
    from em import factors

    target = 7 # Start from the index given in the problem
    while True:
        f1 = factors(target)
        f2 = factors(target + 1)
        for factor in f1:
            if factor in f2:
                f2[factor] += f1[factor]
            else:
                f2[factor] = f1[factor]
        f2[2] -= 1 # 1+2+...+n = n*(n+1)/2

        total = 1
        for factor in f2:
            total *= f2[factor] + 1

        if total >= fmin:
            return target * (target+1) // 2

        target += 1

if __name__ == "__main__":
	print(solve())
