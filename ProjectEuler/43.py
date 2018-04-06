target_primes = [2, 3, 5, 7, 11, 13, 17]
########## Solution ##########
from itertools import permutations

total = 0

for number in permutations(str(d) for d in range(10)):
    if all(int(''.join(number[i+1:i+4])) % prime == 0 for i, prime in enumerate(target_primes)):
        total += int(''.join(number))

print(total)
