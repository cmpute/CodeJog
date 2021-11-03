TARGETS = [1, 10, 100, 1000, 10000, 100000, 1000000]

def generate():
    current = 1
    while True:
        for d in str(current):
            yield d
        current += 1

def solve(targets = TARGETS):
    '''
    Simulation solution
    '''
    product = 1
    for idx, d in enumerate(generate()):
        if idx + 1 in targets:
            product *= int(d)
        if idx > targets[-1]:
            break
    return product

if __name__ == "__main__":
	print(solve())
