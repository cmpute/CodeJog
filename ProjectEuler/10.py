limit = 2000000

pfilter = set(range(2, limit))
for i in range(2, limit): # Sieve of Eratosthenes
    if i not in pfilter:
        continue
    j = i * 2
    while j < limit:
        if j in pfilter:
            pfilter.remove(j)
        j += i
    i += 1
print(sum(pfilter))
