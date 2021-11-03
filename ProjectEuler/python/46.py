
from em import isprime, primes
from itertools import filterfalse, count

def check(number):
	if isprime(number):
		return True # skip non-composite
	for i in count(1):
		s = number - 2 * i * i
		if s <= 0:
			return False
		elif isprime(s):
			return True

def solve():
	return next(filterfalse(check, count(9, 2)))

if __name__ == "__main__":
	print(solve())
