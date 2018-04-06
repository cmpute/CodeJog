LIMIT = 1000

from em import isprime

def count_primes(ab):
    a, b = ab
    if not isprime(b):
        return 0
    counter = 0
    current = b
    while current > 1 and isprime(current):
        current += counter * 2 + 1 + a
        counter += 1
    return counter

def solve(limit = LIMIT):
    '''
    Brute-force solution
    '''
    ab = max(((a, b) for a in range(-limit + 1, limit)
                    for b in range(2, limit)), # b has to be a positive prime to make the result a prime when n=0
                    key=count_primes)
    return ab[0] * ab[1]

if __name__ == "__main__":
	print(solve())
