from em import QuadraticSurd
import math

LIMIT = 10000

def solve(limit=LIMIT):
    counter = 0
    for i in range(2, limit+1):
        sq = math.sqrt(i)
        a0 = int(sq)
        if a0 ** 2 == i: # skip square numbers
            continue

        sq = QuadraticSurd.sqrt(i)
        residual = sq - sq.floor()
        residual_set = { residual }

        period = 1
        while True:
            inv = residual.inverse()
            n = inv.floor()
            residual = inv - n
            if residual in residual_set:
                break
            else:
                residual_set.add(residual)
                period += 1

        if period % 2:
            counter += 1

    return counter

if __name__ == "__main__":
    print(solve())
