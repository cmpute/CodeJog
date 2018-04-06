TARGETS = [2, 3, 5, 7, 11, 13, 17]

def solve(targets = TARGETS):
    from itertools import permutations

    total = 0
    for number in permutations(str(d) for d in range(10)):
        if all(int(''.join(number[i+1:i+4])) % prime == 0 for i, prime in enumerate(targets)):
            total += int(''.join(number))
    return total

if __name__ == "__main__":
	print(solve())
