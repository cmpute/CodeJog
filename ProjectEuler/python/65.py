from em import gcd

TARGET = 100

def solve(target = TARGET):
    repeats = int(target / 3) + 2
    constants = sum(([1, 2*i, 1] for i in range(1, repeats)), [2])[:target]
    
    num = 1
    den = 0
    for c in reversed(constants):
        a = den + c*num
        b = num
        # XXX: a and b are always co-prime judging from the problem description
        g = 1 # g = gcd(a, b)
        num = a // g
        den = b // g

    return sum([int(c) for c in str(num)])

if __name__ == "__main__":
    print(solve())
