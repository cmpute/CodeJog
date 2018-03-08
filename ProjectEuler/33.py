# brute-force
########## Solution ##########
from em import gcd
nominator = denominator = 1
for i in range(10, 100):
    for j in range(10, i):
        if i % 10 == 0 and j % 10 == 0:
            continue
        i0, i1 = i % 10, i // 10
        j0, j1 = j % 10, j // 10
        if (i1 == j0 and i0 * j == i * j1) or (i0 == j1 and i1 * j == i * j0):
            nominator *= j
            denominator *= i

print(denominator // gcd(nominator, denominator))
