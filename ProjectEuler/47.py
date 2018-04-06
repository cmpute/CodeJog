CONSECUTIVE = 4
DISTINCT = 4

from em import factors, primes
from itertools import count
factorset = {}

def solve(consecutive = CONSECUTIVE, distinct = DISTINCT):
    start = 1000
    for i in range(1, consecutive):
        factorset[start - i] = set(factors(start - i).keys())
    for n in count(start):
        factorset[n] = set(factors(n, 50).keys())
        if all(len(factorset[n-i]) >= distinct for i in range(consecutive)):
            return n - distinct + 1

if __name__ == "__main__":
	print(solve())
