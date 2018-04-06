from em import iterpolygonal
from itertools import count

pentagonals = [1]
pentagonalset = set([1])
iterator = iterpolygonal(5)

def preserve(limit):
    while pentagonals[-1] < limit:
        pentagonal = next(iterator)
        pentagonals.append(pentagonal)
        pentagonalset.add(pentagonal)

def solve():
    '''
    Note: if p_n = x, then (24x + 1) should be square number
    '''
    for i in count():
        preserve(2 * pentagonals[i-1])
        p = pentagonals[i]
        for j in range(i):
            p2 = pentagonals[j]
            if (p + p2) in pentagonalset and p - p2 in pentagonalset:
                return p - p2 # p - p2 is roughly increasing

if __name__ == "__main__":
	print(solve())
