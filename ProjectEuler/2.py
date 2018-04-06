LIMIT = 4000000

def solve(limit = LIMIT):
    fib = [1, 1]
    total = 1
    while(fib[-1] < limit):
        if(fib[-1] & 1):
            total += fib[-1]
        fib.append(fib[-1] + fib[-2])
    return total

if __name__ == "__main__":
	print(solve())
