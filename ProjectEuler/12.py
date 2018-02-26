# Alternative solution: Pollard's rho algorithm
fmin = 500
########## Solution ##########
from math import sqrt
primes = [2]
current = 3

def require_prime(max):
    global current
    while primes[-1] <= max:
        flag = True
        for prime in primes:
            if current % prime == 0:
                flag = False
                break
        if flag:
            primes.append(current)
        current += 1

def get_factors(target):
    require_prime(sqrt(target))
    factors = dict()
    for prime in primes:
        while target % prime == 0:
            target = target // prime
            if prime not in factors:
                factors[prime] = 0
            factors[prime] += 1
    return factors

target = 7 # Start from the index given in the problem
while True:
    f1 = get_factors(target)
    f2 = get_factors(target + 1)
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
