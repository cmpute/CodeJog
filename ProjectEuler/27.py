# Brute-force
limit = 1000
########## Solution ##########
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

ab = max(((a, b) for a in range(-limit + 1, limit)
                 for b in range(2, limit)), # b has to be a positive prime to make the result a prime when n=0
                 key=count_primes)
print(ab[0] * ab[1])
