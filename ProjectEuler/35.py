LIMIT = 1000000

def solve(limit = LIMIT):
    from em import primes
    pset = set(primes(limit))

    # be careful for that '71993' is a rotation of `930719`
    result = [prime for prime in pset if all(
                int(str(prime)[i:] + str(prime)[:i]) in pset 
                for i in range(1, len(str(prime)))
            )]
    return len(result)

if __name__ == "__main__":
	print(solve())
