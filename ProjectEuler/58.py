THRES = 0.1

from itertools import count
from em import isprime

def solve(thres = THRES):
    total = 1.
    total_prime = 0
    d1, d2, d3, d4 = 3, 5, 7, 9

    for n in count(1):
        total += 4
        total_prime += isprime(d1) + isprime(d2) + isprime(d3) + isprime(d4)
        d1 += 8*n + 2
        d2 += 8*n + 4
        d3 += 8*n + 6
        d4 += 8*n + 8

        if total_prime / total < thres:
            break

    return 2*n+1

if __name__ == "__main__":
    print(solve())
