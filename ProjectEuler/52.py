def solve():
    from itertools import count
    for i in count(1):
        digits = sorted(str(i))
        if all(sorted(str(i * j)) == digits for j in range(2, 7)):
            return i

if __name__ == "__main__":
	print(solve())
