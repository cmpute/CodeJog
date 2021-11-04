from itertools import count
from collections import defaultdict

PERMS = 5

def solve(perms = PERMS):
    counter = defaultdict(list)
    for i in count(1):
        c = i**3
        stats = defaultdict(int)
        for ch in str(c):
            stats[ch] += 1
        k = frozenset(stats.items())
        counter[k].append(c)
        if len(counter[k]) >= perms:
            return min(counter[k])

if __name__ == "__main__":
    print(solve())
