from itertools import permutations
from em import isprime

def solve():
    '''
    Brute-force solution
    '''

    digits = [str(d) for d in range(1, 10)]
    for size in reversed(range(10)):
        for number in reversed(sorted(permutations(digits[:size]))):
            number = int(''.join(number))
            if isprime(number):
                return number

if __name__ == "__main__":
	print(solve())
