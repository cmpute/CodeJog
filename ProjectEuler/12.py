# Alternative solution: Pollard's rho algorithm
fmin = 500
########## Solution ##########
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

    sum = 1
    for factor in f2:
        sum *= f2[factor] + 1

    if sum >= fmin:
        print(target * (target+1) // 2)
        break
    target += 1
