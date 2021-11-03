LIMIT = 1000000

def solve(limit = LIMIT):
    from em import primes, isprime
    from itertools import count

    lprimes = primes(limit)
    maxlen = 21
    psum = 0
    for plen in count(maxlen, 2):
        if sum(lprimes[:plen]) > limit:
            return psum
        for start in range(len(lprimes) - plen):
            current = sum(lprimes[start:start+plen])
            if current > limit:
                break
            if isprime(current) and plen > maxlen:
                psum = current
                maxlen = plen

if __name__ == "__main__":
	print(solve())
