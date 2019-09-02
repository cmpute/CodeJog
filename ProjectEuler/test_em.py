import timeit
from itertools import islice

from random import randint
import math
import em

prime100 = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
prime50 = [p for p in prime100 if p < 50]
primet = 6469693333
fac123456789 = {3: 2, 3803: 1, 3607: 1}

def test():
    print("----- Unit Test ----- ")
    a = randint(1, 100000)
    b = randint(1, 100000)
    assert em.lb(a) == int(math.log(a, 2)), "lb() assert failed!"
    assert em.log(a, 8) == int(math.log(a, 8)), "log() assert failed!"
    assert em.sqrt(a) == int(math.sqrt(a)), "sqrt() assert failed!"
    assert em.gcd(a, b) == math.gcd(a, b), "powmod() assert failed!"
    assert em.mulmod(a, b, 100) == a * b % 100, "mulmod() assert failed!"
    assert em.powmod(a, b, 100) == pow(a, b, 100), "powmod() assert failed!"
    t = em.randrange(min(a, b), max(a, b))
    assert min(a, b) <= t and t < max(a, b), "randrange() assert failed!"
    print("numeric passed!")

    assert all(prime50[i] == em.primes(50)[i] for i in range(len(prime50)))
    assert all(prime100[i] == em.primes(100)[i] for i in range(len(prime100)))
    em.clearprimes()
    assert all(prime100[i] == em.nprimes(len(prime100))[i] for i in range(len(prime100)))
    plist = list(islice(em.iterprimes(), len(prime100)))
    assert all(prime100[i] == plist[i] for i in range(len(prime100)))
    em.clearprimes()
    assert em.factors(123456789) == fac123456789
    print("prime passed!")

def benchmark():
    print("----- Numeric functions: ----- ")
    print("em.lb: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.lb(a)', setup='from random import randint; import em; a = randint(1e5, 1e6)'),
        timeit.timeit('int(math.log(a, 2))', setup='from random import randint; import math; a = randint(1e5, 1e6)')
    ))
    print("em.log: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.log(a, 8)', setup='from random import randint; import em; a = randint(1e5, 1e6)'),
        timeit.timeit('int(math.log(a, 8))', setup='from random import randint; import math; a = randint(1e5, 1e6)')
    ))
    print("em.sqrt: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.sqrt(a)', setup='from random import randint; import em; a = randint(1e5, 1e6)'),
        timeit.timeit('int(math.sqrt(a))', setup='from random import randint; import math; a = randint(1e5, 1e6)')
    ))
    print("em.gcd: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.gcd(min(a, b), max(a, b))', setup='from random import randint; import em; a = randint(1e5, 1e6); b = randint(1e4, 1e5)'),
        timeit.timeit('gcd(min(a, b), max(a, b))', setup='from random import randint; from math import gcd; a = randint(1e5, 1e6); b = randint(1e4, 1e5)')
    ))
    print("em.mulmod: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.mulmod(a, b, 100)', setup='from random import randint; import em; a = randint(1e5, 1e6); b = randint(1e4, 1e5)'),
        timeit.timeit('a * b % 100', setup='from random import randint; a = randint(1e5, 1e6); b = randint(1e4, 1e5)')
    ))
    print("em.powmod: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.powmod(a, b, 100)', setup='from random import randint; import em; a = randint(1e5, 1e6); b = randint(1e4, 1e5)'),
        timeit.timeit('pow(a, b, 100)', setup='from random import randint; a = randint(1e5, 1e6); b = randint(1e4, 1e5)')
    ))
    print("em.randrange: %.4fs (vs Python %.4fs)" % (
        timeit.timeit('em.randrange(min(a, b), max(a, b))', setup='from random import randint; import em; a = randint(1e5, 1e6); b = randint(1e4, 1e5)'),
        timeit.timeit('randrange(min(a, b), max(a, b))', setup='from random import randint, randrange; a = randint(1e5, 1e6); b = randint(1e4, 1e5)')
    ))

    print("----- Prime generator: ----- ")
    print("Prime under 1e5: %.4fs" % timeit.timeit('em.primes(1e5)', setup='import em', number=1000))
    print("Factorize ~1e3: loop %.4fs, rho %.4fs" % (
        timeit.timeit('em.factors(a, 1e9)', setup='from random import randint; import em; a = randint(1e3, 1.1e3);', number=1000),
        timeit.timeit('em.factors(a, 1)', setup='from random import randint; import em; a = randint(1e3, 1.1e3);', number=1000)
    ))
    print("Factorize ~1e8: loop %.4fs, rho %.4fs" % (
        timeit.timeit('em.factors(a, 1e9)', setup='from random import randint; import em; a = randint(1e3, 1.1e8);', number=1000),
        timeit.timeit('em.factors(a, 1)', setup='from random import randint; import em; a = randint(1e3, 1.1e8);', number=1000)
    ))

if __name__ == "__main__":
    test()
    benchmark()
