from itertools import takewhile, product, permutations
from collections import defaultdict
from em import iterpolygonal

polygon_numbers = defaultdict(list)
number_side = defaultdict(list)
suffix_connect = defaultdict(list)

# use bit vector as side table
def reach(start, end, sidetable=0b111111000, total=0):
    if sidetable == 0:
        if start % 100 == end // 100:
            return [start]
        else:
            return False

    for nextnum in suffix_connect[start % 100]:
        for side in number_side[nextnum]:
            if (sidetable >> side) & 1 == 0: # side set used
                continue
            result = reach(nextnum, end, sidetable & ~(1 << side), total)
            if result: return [start] + result

    return False

def solve():
    # construct tables
    for side in range(3, 9):
        for i in iterpolygonal(side):
            if i < 1000:
                continue
            if i > 10000:
                break
            polygon_numbers[side].append(i)
            suffix_connect[i//100].append(i)
            number_side[i].append(side)

    # enumerate any set of numbers
    initset = 3
    for num in polygon_numbers[initset]:
        result = reach(num, num, 0b111111000 & ~(1 << initset))
        if result:
            return sum(result)

if __name__ == "__main__":
    print(solve())
