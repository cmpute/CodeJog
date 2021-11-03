LIMIT = 10000

from em import factors as ef
from numpy import array, meshgrid, prod

def get_divisors(target):
    factors = ef(target)
    if len(factors) == 0: 
        return [1, target]
    factor_selections = []
    for f in factors:
        factor_selections.append([f ** exp for exp in range(factors[f] + 1)])
    # TODO: save multiply results every two factors
    selections = array(meshgrid(*factor_selections)).reshape(len(factors), -1)
    products = prod(selections, axis=0)
    return products

def solve(limit = LIMIT):
    dn = [None, None, None, None]
    for i in range(4, limit + 1):
        dsum = sum(get_divisors(i)[:-1])
        dn.append(dsum)
    total = 0
    for num in range(len(dn)):
        if num is None or dn[num] is None: continue
        if dn[num] > limit: continue
        if num == dn[num]: continue
        if num == dn[dn[num]]:
            total += num
    return total

if __name__ == "__main__":
	print(solve())
