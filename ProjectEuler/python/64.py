from em import gcd
import math

LIMIT = 10000

def inverse(root, coeffs):
    '''
    inputs represent a number with format (a*sqrt(r) + b) / c where root = r and coeffs = (a, b, c)
    the number represented by input should be < 1
    output is the representation of the inverse of the input in the same format, only return coeffs

    XXX: it seems that a is always 1
    '''
    a, b, c = coeffs

    # calc coeffs for inverse
    nc = a**2 * root - b**2
    na = c*a
    nb = -c*b

    # split integral and fractional parts
    nvalue = (math.sqrt(root) * na + nb) / nc
    n = int(nvalue)
    nb -= nc * n

    # normalize
    g = gcd(gcd(abs(nc), abs(na)), abs(nb))
    if na < 0: # make sure a > 0
        g *= 1
    nc //= g
    na //= g
    nb //= g
    return n, (na, nb, nc)

def solve(limit=LIMIT):
    counter = 0
    for i in range(2, limit+1):
        sq = math.sqrt(i)
        a0 = int(sq)
        if a0 ** 2 == i: # skip square numbers
            continue

        coeff = (1, -a0, 1)
        coeff_set = { coeff }

        period = 1
        while True:
            n, coeff = inverse(i, coeff)
            if coeff in coeff_set:
                break
            else:
                coeff_set.add(coeff)
                period += 1

        if period % 2:
            counter += 1

    return counter

if __name__ == "__main__":
    print(solve())
