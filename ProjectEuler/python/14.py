LIMIT = 1000000

path = {1: 4}
length = {1: 0}

def expand(n):
    if n in path:
        return
    if n & 1: # odd
        target = 3*n + 1
    else:
        target = n >> 1
    expand(target)
    path[n] = target
    length[n] = length[target] + 1

def solve(limit = LIMIT):
    for current in range(1, limit):
        expand(current)
    return max(length, key=length.get)

if __name__ == "__main__":
	print(solve())
