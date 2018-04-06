from em import iterpolygonal

def join():
    # Every hexagonal number is also a triangular number
    penta = iterpolygonal(5)
    hexa = iterpolygonal(6)
    cpenta = -1
    chexa = -2
    while True:
        if cpenta < chexa:
            cpenta = next(penta)
        elif chexa < cpenta:
            chexa = next(hexa)
        else:
            yield cpenta
            cpenta += 1

def solve():
    result = join()
    next(result) # 1
    next(result) # number in the problem
    return next(result)

if __name__ == "__main__":
	print(solve())
