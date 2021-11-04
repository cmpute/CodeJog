from itertools import count
from math import ceil

def solve():
    counter = 0
    for i in count(1): # number of digits / power
        found = False
        start = int(ceil(10**((i-1)/i)))
        for j in count(start):
            d = j**i
            if d >= 10**i:
                break
            counter += 1
            found = True
        if not found:
            break
    return counter

if __name__ == "__main__":
    print(solve())
