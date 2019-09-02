from em import iterprimes, primes, isprime, memoize

# primes that fulfill the property is far more less, so store true values
cache = {2: list(), 3: list(), 4: list()}
primes(1e6, True)

@memoize
def test2(a, b):
    return isprime(int(str(a) + str(b))) and isprime(int(str(b) + str(a)))

def testn(a, l):
    for p in l:
        if not test2(a, p):
            return False
    return True
            
def solve():
    # In this order we prevent duplicate computation
    for a in iterprimes():
        for plist in cache[4]:
            if testn(a, plist):
                return sum([a] + plist)
        for plist in cache[3]:
            if testn(a, plist):
                cache[4].append([a] + plist)
        for plist in cache[2]:
            if testn(a, plist):
                cache[3].append([a] + plist)
        for p in primes(a):
            if test2(a, p):
                cache[2].append([a, p])

if __name__ == '__main__':
    print(solve())