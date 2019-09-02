CONSECUTIVE = 4
DISTINCT = 4

from em import sqrt, primes
from itertools import count
factorset = {}

def pfactor(target):
    flist = set()
    for prime in primes(sqrt(target) + 1):
        if target % prime == 0:
            flist.add(prime)
            target //= prime
            while target % prime == 0:
                target //= prime

        if target == 1 or prime > target:
            break
    if target != 1:
        flist.add(target)

    return flist

def solve(consecutive = CONSECUTIVE, distinct = DISTINCT):
    start = 1000
    for i in range(1, consecutive):
        factorset[start - i] = pfactor(start - i)
    for n in count(start):
        factorset[n] = pfactor(n)
        if all(len(factorset[n-i]) >= distinct for i in range(consecutive)):
            return n - distinct + 1

if __name__ == "__main__":
	print(solve())
