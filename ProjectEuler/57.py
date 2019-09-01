LIMIT = 1000

def solve(limit = LIMIT):
    num = 3
    den = 2
    count = 0
    for _ in range(limit):
        tden = num + den
        tnum = den + tden
        num, den = tnum, tden
        if len(str(num)) > len(str(den)):
            count += 1

    return count

if __name__ == "__main__":
    print(solve())