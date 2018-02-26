# Brute-force and use python built-in
target = 100
########## Solution ##########
from math import factorial
print(sum(int(c) for c in str(factorial(100))))
