limit = 1000000
########## Solution ##########
from em import primes
pset = set(primes(limit))
count = 0
results = set()

# be careful for that '71993' is a rotation of `930719`
result = [prime for prime in pset if all(
            int(str(prime)[i:] + str(prime)[:i]) in pset 
            for i in range(1, len(str(prime)))
         )]
print(len(result))
