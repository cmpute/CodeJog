from itertools import islice
from em import isprime, iterprimes

def check(target):
    # left truncatable
    divider = 10
    while divider <= target:
        if not isprime(target % divider):
            return False
        divider *= 10
    # right truncatable
    target //= 10
    while target > 0:
        if not isprime(target):
            return False
        target //= 10
    return True

def solve():
    return sum(islice(filter(check, iterprimes()), 4, 15))

if __name__ == "__main__":
	print(solve())
