target = 10001
########## Solution ##########
primes = [2]
current = 3
while len(primes) < target:
    flag = True
    for prime in primes:
        if current % prime == 0:
            flag = False
            break
    if flag:
        primes.append(current)
    current += 1
print(primes[-1])
