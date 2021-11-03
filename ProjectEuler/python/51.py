TARGET = 8

def solve(target = TARGET):
    from em import iterprimes, isprime
    from itertools import product

    families = set()
    digits = [str(d) for d in range(10)]

    for prime in iterprimes():
        if prime < 10000:
            continue
        sprime = str(prime)

        masks = [(digit, '*') for digit in sprime]
        for mask in product(*masks):
            family = ''.join(mask)
            if family == sprime: continue # no replaceable digits
            if len(set(sprime[i] for i, d in enumerate(family) if d == '*')) > 1:
                continue

            if family in families:
                continue
            else:
                # This condition is awkwardly implied by the problem description.
                # Without this condition, the result is 111109
                if mask[0] == '*':
                    replacer = digits[1:] # remove 0
                else:
                    replacer = digits

                total = sum(isprime(int(family.replace('*', d))) for d in replacer)
                if total >= target:
                    return prime
                families.add(family)

if __name__ == "__main__":
	print(solve())
