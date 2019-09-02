# distutils: language = c++
# cython: boundscheck=False, wraparound=False, nonecheck=False, cdivision=True

'''
Math library for Project Euler (em: Euler Math)
Copyright (C) 2018- Jacob Zhong

Compile: cythonize -i em.pyx
'''

DEF LARGE_INT_SUPPORT = False

cimport cython
from cython.operator cimport dereference as deref, preincrement as inc
from libc.stdlib cimport rand, RAND_MAX
from libcpp.vector cimport vector as clist
from libcpp.map cimport map as cmap
from libcpp.set cimport set as cset
from cpython cimport int as pyint

ctypedef unsigned long long ulong
IF LARGE_INT_SUPPORT:
    # XXX: Currently PyInt is considered as long in template params
    ctypedef pyint clong
ELSE:
    ctypedef ulong clong

# ----- Numeric Functions -----
cpdef int lb(clong target):
    '''
    Returns floor(log(2, target))
    If target is 0, then -1 is returned
    '''
    cdef int counter = -1
    while target > 0:
        target >>= 1
        counter += 1
    return counter

cpdef int log(clong target, clong base):
    '''
    Returns floor(log(base, target))
    If target is 0, then -1 is returned
    '''
    cdef int counter = -1
    while target > 0:
        target //= base
        counter += 1
    return counter

cpdef clong sqrt(clong target):
    '''
    Returns floor(sqrt(target))

    # Reference
    *Computing Integer Square Roots*, James Ulery
    '''
    cdef:
        unsigned int hbit = lb(target) >> 1
        clong test
        clong hnum = 1 << hbit
        clong result = 0
    
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

cpdef clong gcd(clong a, clong b):
    '''
    Returns gcd(a, b)
    '''
    if b > a:
        return gcd(b, a)
    if a % b == 0:
        return b
    return gcd(b, a % b)

cpdef clong mulmod(clong a, clong b, clong mod):
    '''
    Return (a * b) % mod, even works for very large numbers
    '''
    cdef clong result = 0;
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

cpdef clong powmod(clong a, clong exp, clong mod):
    '''
    Return (a ^ exp) % mod, even works for very large numbers
    
    Note: Python also has efficient built-in implementation: pow(a, exp, mod)
    '''
    cdef:
        clong multi = a
        clong result = 1
    if exp == 1:
        return a % mod

    multi %= mod
    while exp > 0:
        if exp & 1:
            result = mulmod(result, multi, mod)
        multi = mulmod(multi, multi ,mod)
        exp >>= 1
    return result

cpdef clong randrange(clong a, clong b):
    '''
    Fast version if random.randrange()
    '''
    cdef clong result = 1, diff = b - a
    if diff <= 0:
        raise ValueError("Invalid range for randrange!")

    while diff > RAND_MAX:
        diff //= RAND_MAX
        result *= rand()

    result *= (rand() % diff)
    return result + a

# ----- Prime utilities -----

# global cache
cdef clist[ulong] prime_list = [2] # Add more primes here to fasten sieve
cdef clong prime_current = 3
cdef clong WARN_SIZE = 2**25

cpdef int fprime(clong x, int lo=0, int hi=0):
    """
    C version of bisect.bisect_left(prime_list, x, lo, hi)
    """
    cdef int mid
    if hi == 0:
        hi = prime_list.size()

    while lo < hi:
        mid = (lo + hi) >> 1
        if prime_list[mid] < x: lo = mid+1
        else: hi = mid
    return lo

cpdef list primes(clong limit, bint suppress=False):
    '''
    Returns all primes **under** limit. The primes are sorted. The `report` switch is used for
        C optimization since the list conversion is expensive
    
    # Reference
    https://stackoverflow.com/questions/2068372/fastest-way-to-list-all-primes-below-n/3035188#3035188

    # Note
    List primes works very slow for limit larger than 2^25 and won't work if limit > 2^30
    '''
    global prime_current
    cdef:
        # Only sieve numbers from current prime to limit
        cset[ulong] sieve
        clong multi, prime

    if limit <= prime_current and not suppress:
        return prime_list[:fprime(limit)]

    # Initialize linear Sieve (with 2 pre-filtered)
    for multi in range(prime_current | 1, limit, 2):
        sieve.insert(sieve.end(), multi) # hopefully O(1)

    # sieve with existing primes
    piter = prime_list.begin()
    inc(piter) # skip 2
    while piter != prime_list.end():
        prime = deref(piter)
        multi = prime * prime
        if multi < prime_current:
            multi = prime * ((prime_current // prime) | 1)
        while multi < limit:
            if sieve.find(multi) != sieve.end():
                sieve.erase(<ulong> multi)
            multi += 2 * prime
        inc(piter)

    # sieve with new primes
    for prime in range(prime_current | 1, sqrt(limit) + 1, 2):
        if sieve.find(prime) == sieve.end():
            continue
        multi = prime * prime
        while multi < limit:
            if sieve.find(multi) != sieve.end():
                sieve.erase(<ulong> multi)
            multi += 2 * prime

    for prime in sieve:
        prime_list.push_back(prime)
    prime_current = limit

    if not suppress:
        return prime_list

cpdef list nprimes(clong count):
    '''
    Returns primes of certain amount counting from 2. The primes are sorted.
    '''
    while prime_list.size() < count:
        primes(prime_current * count // prime_list.size(), True)
    return prime_list

def iterprimes():
    '''
    Returns an endless iterator of primes.
    '''
    cdef clong current = 0
    while True:
        if current >= prime_list.size():
            primes(prime_current << 1, True)
        yield prime_list[current]
        current += 1

cpdef clearprimes():
    '''
    Clear primes to prevent performance influence of factorization
    '''
    global prime_current
    prime_list = [2]
    prime_current = 3

cpdef bint isprime(clong target, int confidence=5):
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
        clong x, pre
        clong u = target - 1
        int idx
    if target <= 1:
        return False
    
    # find in list
    idx = fprime(target)
    if idx != prime_list.size():
        return prime_list[idx] == target

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

cpdef clong divisor(clong target):
    '''
    Return a proper divisor of target (randomly), even works for very large numbers

    # Reference
    Pollard's rho algorithm
    http://blog.csdn.net/z690933166/article/details/9865755
    '''
    if isprime(target):
        raise ValueError('Cannot find a proper divisor of a prime')

    cdef:
        clong t, a, b, i, j, d
        clong prime

    if lb(target) < 10: # regress to naive method
        t = sqrt(target)
        primes(t, True)
        piter = prime_list.begin()
        while piter != prime_list.end():
            prime = deref(piter)
            if prime > t:
                raise ValueError('Cannot find a proper divisor of a prime')
            if target % prime == 0:
                return prime
            inc(piter)

    # Otherwise use Rho algorithm
    prime = target
    while prime >= target:
        a = b = randrange(1, target)
        t = randrange(1, target)
        i, j = 1, 2
        while True:
            i += 1
            a = (mulmod(a, a, target) + t) % target
            if a == b:
                prime = target
                break
            d = gcd(abs(b - a), target)
            if 1 < d or d < a:
                prime = d
                break
            if i == j:
                b = a
                j <<= 1
    return prime

cpdef dict factors(clong target, int threshold=int(1e6)):
    '''
    Return the prime factors of target.

    # Parameters
    threshold: The algorithm will regress to naive one when under the threshold (exponential).
    '''
    cdef clong prime, factor
    cdef cmap[ulong, int] f1, f2

    if isprime(target):
        return {target: 1}
    
    if lb(target) < threshold: # regress to naive method
        primes(sqrt(target) + 1, True)
        piter = prime_list.begin()
        while piter != prime_list.end():
            prime = deref(piter)
            while target % prime == 0: 
                target //= prime 
                if f1.find(prime) == f1.end(): 
                    f1[prime] = 0 
                f1[prime] += 1
            if target == 1:
                break
            inc(piter)

        if target != 1:
            f1[target] = 1
        return f1 

    prime = 1
    while prime == 1:
        try:
            # If here is an error occurred, that is isprime() failed to judge a prime
            prime = divisor(target)
        finally:
            continue
    
    f1 = factors(prime, threshold)
    f2 = factors(target // prime, threshold)
    fiter = f1.begin() # fiter.first: factor, fiter.second: factor count
    while fiter != f1.end():
        factor = deref(fiter).first
        if f2.find(factor) != f2.end():
            f2[factor] += deref(fiter).second
        else:
            f2[factor] = deref(fiter).second
        inc(fiter)
    return f2

# ----- Miscellaneous -----

cdef class memoize(object):
    cdef object func
    cdef dict cache

    def __init__(self, func):
        self.func = func
        self.cache = {}
    
    def __call__(self, *args):
        if args in self.cache:
            return self.cache[args]
        else:
            val = self.func(*args)
            self.cache[args] = val
            return val

def iterpolygonal(clong s):
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
    cdef clong n = 0
    cdef clong result = 0
    while True:
        result += n * s + 1
        n += 1
        yield result
