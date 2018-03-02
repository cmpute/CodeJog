# Brute-force
index = 1000000
########## Solution ##########
from itertools import permutations
combination = permutations(range(10))
while index > 0:
    index -= 1
    result = next(combination)
print(''.join(str(d) for d in result))
