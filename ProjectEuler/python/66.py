from em import sqrt
from itertools import count

LIMIT = 1000

def is_sq(v):
    sq = sqrt(v)
    return sq * sq == v

def solve(limit=LIMIT):
    result = 1
    for d in range(limit+1):
        print(d)
        if is_sq(d):
            continue

        for y in count(1):
            x2 = 1 + d*y*y
            if is_sq(x2):
                result = max(sqrt(x2), result)
                break
    return result

if __name__ == "__main__":
    print(solve(100))