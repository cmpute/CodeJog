from math import ceil, sqrt
target = 600851475143
limit = ceil(sqrt(target))
pfilter = set(range(limit))
factors = set()
for i in range(2, limit): # Sieve of Eratosthenes
    if i not in pfilter:
        continue
    j = i
    while j < limit:
        if j in pfilter:
            pfilter.remove(j)
        j += i
    while target % i == 0:
        target = target // i
        factors.add(i)
    if(target < i):
        break
    i += 1
print(max(factors))
