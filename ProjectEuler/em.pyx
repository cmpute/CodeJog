'''
Math library for Project Euler
Copyright (C) 2018- Jacob Zhong
'''

from cpython cimport int as pyint
from random import randrange
from bisect import bisect_left

def lb(pyint target):
    '''
    Returns floor(log(2, target))
    If target is 0, then -1 is returned
    '''
    cdef int counter = -1
    while target > 0:
        target >>= 1
        counter += 1
    return counter

def log(pyint target, pyint base):
    '''
    Returns floor(log(base, target))
    If target is 0, then -1 is returned
    '''
    cdef int counter = -1
    while target > 0:
        target //= base
        counter += 1
    return counter

def sqrt(pyint target):
    '''
    Returns floor(sqrt(target))

    # Reference
    *Computing Integer Square Roots*, James Ulery
    '''
    cdef:
        unsigned int hbit = lb(target) >> 1
        unsigned int test
        unsigned int hnum = 1 << hbit
        unsigned int result = 0
    
    while True:
        test = ((result << 1) + hnum) << hbit
        if target >= test:
            result += hnum
            target -= test
        if hnum == 0:
            break
        hbit -= 1
        hnum >>= 1
    return result

# Add more primes here to fasten sieve
cdef list prime_list = [2]
cdef pyint prime_current = 3

def primes(pyint limit, loop_predicate=None):
    '''
    Returns all primes under limit. The primes are sorted.

    # Params (for optimization)
    loop_predicate: A function to judge whether the loop will be stopped immediately.
        If loop_predicate(prime) is False, then the loop ends.
    
    # Reference
    https://stackoverflow.com/questions/2068372/fastest-way-to-list-all-primes-below-n/3035188#3035188

    # Notes
    Works awkward if limit > 2^25 and won't work if limit > 2^30
    '''
    global prime_current
    cdef:
        set sieve = set(range(prime_current | 1, limit, 2)) # can be an array
        pyint multi
    if(limit <= prime_current):
        # prevent to execute this because list slicing is relatively expensive
        return prime_list[:bisect_left(prime_list, limit)]

    # Linear Sieve (with 2 pre-filtered)
    for prime in prime_list[1:]: # skip 2
        multi = prime * prime
        if multi < prime_current:
            multi = prime * ((prime_current // prime) | 1)
        while multi < limit:
            if multi in sieve:
                sieve.remove(multi)
            multi += 2 * prime

    for prime in range(prime_current | 1, sqrt(limit) + 1, 2):
        if prime not in sieve:
            continue
        multi = prime * prime
        while multi < limit:
            if multi in sieve:
                sieve.remove(multi)
            multi += 2 * prime
        if loop_predicate and not loop_predicate(prime):
            break

    prime_list.extend(sorted(sieve))
    prime_current = limit
    
    # loop the rest of the primes
    if loop_predicate:
        loop_start = sqrt(limit) + 1
        for prime in prime_list:
            if prime < loop_start:
                continue
            if not loop_predicate(prime):
                break

    return prime_list

def nprimes(pyint count, loop_predicate=None):
    '''
    Returns primes of certain amount counting from 2. The primes are sorted.
    '''
    while len(prime_list) < count:
        primes(prime_current * count // len(prime_list), loop_predicate)
    return prime_list

def iterprimes():
    '''
    Returns an endless iterator of primes.
    '''
    cdef pyint current = 0
    while True:
        if current >= len(prime_list):
            primes(prime_current << 1)
        yield prime_list[current]
        current += 1

def clearprimes():
    '''
    Clear primes to prevent performance influence of factorization
    '''
    global prime_current
    prime_list = [2]
    prime_current = 3

def gcd(pyint a, pyint b):
    '''
    Returns gcd(a, b)
    '''
    if b > a:
        return gcd(b, a)
    if a % b == 0:
        return b
    return gcd(b, a % b)

def mulmod(pyint a, pyint b, pyint mod):
    '''
    Return (a * b) % mod, even works for very large numbers
    '''
    cdef pyint result = 0;
    a %= mod
    b %= mod
    while b > 0:
        if b & 1:
            result += a 
            result %= mod
        a <<= 1
        if a >= mod:
            a %= mod
        b >>= 1
    return result

def powmod(pyint a, pyint exp, pyint mod):
    '''
    Return (a ^ exp) % mod, even works for very large numbers
    
    Implementation
    --------------
    cdef:
        pyint multi = a
        pyint result = 1
    if exp == 1:
        return a % mod
    a %= mod;
    while exp > 0:
        if exp & 1:
            result = mulmod(result, multi, mod)
        multi = mulmod(multi, multi ,mod)
        exp >>= 1
    return result
    '''
    return pow(a, exp, mod)

def isprime(pyint target, int confidence=5):
    '''
    Return whether target is a prime, even works for very large numbers and very fast

    # Parameter
    confidence: The larger it is, the result is more reliable

    # Reference
    Miller–Rabin primality test
    http://www.cnblogs.com/vongang/archive/2012/03/15/2398626.html
    '''
    cdef:
        int shift = 0
        pyint x, pre
        pyint u = target - 1
        int idx
    if target <= 1:
        return False
    
    # find in list
    idx = bisect_left(prime_list, target)
    if idx != len(prime_list) and prime_list[idx] == target:
        return True

    while(not u & 1):
        shift += 1
        u >>= 1

    for i in range(confidence):
        x = randrange(2, target)
        if x % target == 0:
            continue
        x = powmod(x, u, target)
        pre = x

        for j in range(shift):
            x = mulmod(x, x, target)
            if x == 1 and pre != 1 and pre != target-1:
                return False
            pre = x

        if x != 1:
            return False
    return True

def divisor(pyint target):
    '''
    Return a proper divisor of target (randomly), even works for very large numbers

    # Reference
    Pollard's rho algorithm
    http://blog.csdn.net/z690933166/article/details/9865755
    '''
    if isprime(target):
        raise ValueError('Cannot find a proper divisor of a prime')

    cdef:
        pyint p = target
        pyint t, a, b, i, j, d

    if lb(target) < 10: # regress to naive method
        t = sqrt(target)
        for prime in primes(t):
            if prime > t:
                raise ValueError('Cannot find a proper divisor of a prime')
            if target % prime == 0:
                return prime

    # Otherwise use Rho algorithm
    while p >= target:
        a = b = randrange(1, target)
        t = randrange(1, target)
        i, j = 1, 2
        while True:
            i += 1
            a = (mulmod(a, a, target) + t) % target
            if a == b:
                p = target
                break
            d = gcd(abs(b - a), target)
            if 1 < d or d < a:
                p = d
                break
            if i == j:
                b = a
                j <<= 1
    return p

def factors(pyint target, int threshold=100):
    '''
    Return the prime factors of target. (Use built-in)

    Parameters
    ----------
    threshold: The algorithm will regress to naive one when under the threshold (exponential).
    '''
    cdef pyint p = 1
    cdef dict f1, f2

    if isprime(target):
        return {target: 1}
    
    if lb(target) < threshold: # regress to naive method
        f1 = dict() 
        for prime in primes(sqrt(target) + 1):
            while target % prime == 0: 
                target //= prime 
                if prime not in f1: 
                    f1[prime] = 0 
                f1[prime] += 1
            if target == 1:
                break
        if target != 1:
            f1[target] = 1
        return f1 

    while p == 1:
        try:
            # If here is an error occurred, that is isprime() failed to judge a prime
            p = divisor(target)
        finally:
            continue
    
    f1 = factors(p, threshold)
    f2 = factors(target // p, threshold)
    for factor in f1:
        if factor in f2:
            f2[factor] += f1[factor]
        else:
            f2[factor] = f1[factor]
    return f2

def iterpolygonal(pyint s):
    '''
    Returns an endless iterator of polygonal numbers

    Parameters
    ----------
    s : The number of sides

    Notes
    -----
    Triangular numbers: s = 3
    Square numbers: s = 4
    Pentagonal numbers: s = 5
    Hexagonal numbers: s = 6

    Reference
    ---------
    https://en.wikipedia.org/wiki/Polygonal_number
    '''
    s -= 2
    cdef pyint n = 0
    cdef pyint result = 0
    while True:
        result += n * s + 1
        n += 1
        yield result
