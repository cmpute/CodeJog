LIMIT = 100

def solve(limit = LIMIT):
    maxnum = 0
    for a in range(2, limit+1):
        at = 1
        for _ in range(1, limit+1):
            at = at * a
            sumnum = sum(int(d) for d in str(at))
            if sumnum > maxnum:
                maxnum = sumnum
                print(a, _)
    return maxnum

if __name__ == "__main__":
    print(solve())